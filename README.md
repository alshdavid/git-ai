# Git AI Tools

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
```