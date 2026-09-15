# Git AI Tools

This is a CLI tool that uses AI to automatically summarize commit messages and PRs.

```
git(main) $ git ai help

Usage: git-ai [OPTIONS] --openai-api-url <OPENAI_API_URL> --model <MODEL_ID> <COMMAND>

Commands:
  commit        Create a commit with an automatically generated summary
  pull-request  Create a GitHub PR with any automatically generated title and description
  help          Print this message or the help of the given subcommand(s)

Options:
      --openai-api-url <OPENAI_API_URL>
          Base URL for OpenAI-compatible API endpoints [env: OPENAI_API_BASE=<redacted>]
      --openai-api-token <OPENAI_API_TOKEN>
          API token for OpenAI or compatible provider [env: OPENAI_API_KEY=<redacted>]
      --model <MODEL_ID>
          Model ID to use (e.g., gpt-4o, deepseek-v4-flash) [env: GIT_AI_MODEL_ID=deepseek/deepseek-flash]
      --gh-token <GH_TOKEN>
          GitHub Personal Access Token [env: GH_TOKEN=<redacted>]
  -h, --help
          Print help

git:(main) $ git branch -b foo
git:(foo) $ git add .
git(foo) $ git ai commit

Generate a one-line Conventional Commit message based on this diff. Output ONLY the commit message text, no markdown, no quotes.

<summary>
added README.md +24 -0
</summary>

<diffs>
<file>
--- a/README.md
+++ b/README.md
@@ -0,0 +1,24 @@
+# Git AI Tools
+
+```
+$ git ai help
+
+Usage: git-ai [OPTIONS] --openai-api-url <OPENAI_API_URL> --model <MODEL_ID> <COMMAND>
+
+Commands:
+  commit        Create a commit with an automatically generated summary
+  pull-request  Create a GitHub PR with any automatically generated title and description
+  help          Print this message or the help of the given subcommand(s)
+
+Options:
+      --openai-api-url <OPENAI_API_URL>
+          Base URL for OpenAI-compatible API endpoints [env: OPENAI_API_BASE=<redacted>]
+      --openai-api-token <OPENAI_API_TOKEN>
+          API token for OpenAI or compatible provider [env: OPENAI_API_KEY=<redacted>]
+      --model <MODEL_ID>
+          Model ID to use (e.g., gpt-4o, deepseek-v4-flash) [env: GIT_AI_MODEL_ID=deepseek/deepseek-flash]
+      --gh-token <GH_TOKEN>
+          GitHub Personal Access Token [env: GH_TOKEN=<redacted>]
+  -h, --help
+          Print help
+```
</file>
</diffs>

docs: add README documenting git-ai CLI usage and options

[main 7f0a852] docs: add README documenting git-ai CLI usage and options
 1 file changed, 24 insertions(+)
 create mode 100644 README.md

git(foo) $ git ai pr

Using base branch 'main'
----------------------------------------
Target Branch: main
Generated Title: docs: add README for git-ai CLI usage
Generated Body:
- Add a new README.md introducing the Git AI Tools project.
- Document the `git ai` command-line usage, including the `commit`, `pull-request`, and `help` subcommands.
- List CLI options and their environment variables for API URL, token, model ID, and GitHub token.
- Include an example walkthrough showing `git ai commit` and `git ai pr` in action.
----------------------------------------
Pushing branch 'foo' to remote...
Enumerating objects: 4, done.
Counting objects: 100% (4/4), done.
Delta compression using up to 10 threads
Compressing objects: 100% (3/3), done.
Writing objects: 100% (3/3), 1.09 KiB | 1.09 MiB/s, done.
Total 3 (delta 1), reused 0 (delta 0), pack-reused 0 (from 0)
remote: Resolving deltas: 100% (1/1), completed with 1 local object.
remote: 
remote: Create a pull request for 'foo' on GitHub by visiting:
remote:      https://github.com/alshdavid/git-ai/pull/new/foo
remote: 
To github.com:alshdavid/git-ai.git
 * [new branch]      foo -> foo
branch 'foo' set up to track 'origin/foo'.
Creating Pull Request...

Creating pull request for foo into main in alshdavid/git-ai

https://github.com/alshdavid/git-ai/pull/2
PR created successfully!
```

# Installation

# Linux AMD64
curl -L --url https://github.com/alshdavid/git-ai/releases/latest/download/git-ai-linux-amd64.tar.gz | tar -xvzf - -C $HOME/.local/bin

# Linux ARM64
curl -L --url https://github.com/alshdavid/git-ai/releases/latest/download/git-ai-linux-arm64.tar.gz | tar -xvzf - -C $HOME/.local/bin

# MacOS ARM64 (Apple Silicon)
curl -L --url https://github.com/alshdavid/git-ai/releases/latest/download/git-ai-macos-arm64.tar.gz | tar -xvzf - -C $HOME/.local/bin 

# MacOS AMD64 (Intel)
curl -L --url https://github.com/alshdavid/git-ai/releases/latest/download/git-ai-macos-amd64.tar.gz | tar -xvzf - -C $HOME/.local/bin

# Add to PATH if not already there:
echo "\nexport \PATH=\$PATH:\$HOME/.local/bin\n" >> $HOME/.zshrc
echo "\nexport \PATH=\$PATH:\$HOME/.local/bin\n" >> $HOME/.bashrc