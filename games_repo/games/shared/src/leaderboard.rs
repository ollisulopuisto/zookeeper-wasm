use serde::{de::DeserializeOwned, Serialize};

pub fn load_list<T, F>(max_len: usize, mut loader: F) -> Vec<T>
where
    T: DeserializeOwned,
    F: FnMut(*mut u8, u32) -> u32,
{
    let mut buffer = vec![0u8; max_len];
    let len = loader(buffer.as_mut_ptr(), buffer.len() as u32);
    if len == 0 || len as usize > buffer.len() {
        return Vec::new();
    }

    serde_json::from_slice(&buffer[..len as usize]).unwrap_or_default()
}

pub fn save_list<T, F>(entries: &[T], mut saver: F)
where
    T: Serialize,
    F: FnMut(*const u8, u32),
{
    if let Ok(json_bytes) = serde_json::to_vec(entries) {
        saver(json_bytes.as_ptr(), json_bytes.len() as u32);
    }
}

#[cfg(test)]
mod tests {
    use super::{load_list, save_list};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Default)]
    struct Entry {
        score: u32,
    }

    #[test]
    fn roundtrip_json() {
        let mut saved = Vec::new();
        let entries = vec![Entry { score: 42 }];
        save_list(&entries, |ptr, len| {
            let bytes = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
            saved = bytes.to_vec();
        });

        let loaded = load_list::<Entry, _>(128, |ptr, max| {
            let n = saved.len().min(max as usize);
            let out = unsafe { std::slice::from_raw_parts_mut(ptr, n) };
            out.copy_from_slice(&saved[..n]);
            n as u32
        });

        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].score, 42);
    }
}
