# gh-lens

CLI to analyze your activity on GitHub.

## Install

```console
$ cargo install gh-lens
```

## Usage

Set the GitHub PAT as an environment variable before running the command.

```console
$ export GITHUB_TOKEN = xxx
```

```console
$ gh-lens prs --repo hirokisan/gh-lens --start-date 2024-11-12 --end-date 2024-11-18 | jq .
{
  "start_date": "2024-11-12",
  "end_date": "2024-11-18",
  "prs_count": 1,
  "prs_summaries": [
    {
      "url": "https://github.com/hirokisan/gh-lens/pull/1",
      "author": "hirokisan",
      "comments_count": 0,
      "reviewee_comments_count": 0,
      "reviewer_comments_count": 0,
      "commits_count": 2,
      "changed_files_count": 13,
      "created_at": "2024-11-15T12:17:52Z",
      "first_contacted_at": null,
      "approved_at": null,
      "merged_at": "2024-11-18T12:34:51Z"
    }
  ]
}
```
