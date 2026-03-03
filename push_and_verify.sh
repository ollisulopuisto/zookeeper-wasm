#!/bin/bash
# push_and_verify.sh: A script to commit, push, and monitor GitHub Actions.

MESSAGE=$1
if [ -z "$MESSAGE" ]; then
    echo "Usage: ./push_and_verify.sh "commit message""
    exit 1
fi

git add .
git commit -m "$MESSAGE"
git push origin master

echo "Pushed. Waiting for GitHub Actions to start..."
sleep 5

# Get the latest run ID
RUN_ID=$(gh run list --limit 1 --json databaseId --jq '.[0].databaseId')

echo "Watching Run ID: $RUN_ID"
gh run watch $RUN_ID

# Check final status
STATUS=$(gh run view $RUN_ID --json conclusion --jq '.conclusion')

if [ "$STATUS" == "success" ]; then
    echo "✅ Build successful!"
    exit 0
else
    echo "❌ Build failed. Fetching logs..."
    gh run view $RUN_ID --log-failed
    exit 1
fi
