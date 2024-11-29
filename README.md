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
  "comments_count": {
    "sum": 0,
    "average": 0.0
  },
  "commits_count": {
    "sum": 2,
    "average": 2.0
  },
  "changed_files_count": {
    "sum": 13,
    "average": 13.0
  },
  "time_to_first_contacted": {
    "average": 0.0
  },
  "time_to_approved": {
    "average": 0.0
  },
  "time_to_merged": {
    "average": 260219.0
  },
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

```console
gh-lens issues --repo hirokisan/bybit --start-date 2024-01-01 --end-date 2024-10-31 | jq .
{
  "start_date": "2024-01-01",
  "end_date": "2024-10-31",
  "issues_count": 5,
  "assigns_count": 1,
  "comments_count": {
    "sum": 12,
    "average": 2.4
  },
  "time_to_closed": {
    "average": 339269.2
  },
  "issues_summaries": [
    {
      "url": "https://github.com/hirokisan/bybit/issues/190",
      "author": "id-petrov",
      "assignees": null,
      "participants": [
        "id-petrov",
        "hirokisan"
      ],
      "comments_count": 1,
      "created_at": "2024-10-28T20:10:43Z",
      "closed_at": "2024-10-29T05:39:09Z"
    },
    {
      "url": "https://github.com/hirokisan/bybit/issues/181",
      "author": "austymenko",
      "assignees": null,
      "participants": [
        "austymenko",
        "hirokisan"
      ],
      "comments_count": 5,
      "created_at": "2024-07-06T03:31:11Z",
      "closed_at": "2024-07-13T13:29:15Z"
    },
    {
      "url": "https://github.com/hirokisan/bybit/issues/175",
      "author": "apeman76",
      "assignees": null,
      "participants": [
        "apeman76",
        "hirokisan"
      ],
      "comments_count": 2,
      "created_at": "2024-06-12T14:13:08Z",
      "closed_at": "2024-06-23T07:57:42Z"
    },
    {
      "url": "https://github.com/hirokisan/bybit/issues/171",
      "author": "s-prosvirnin",
      "assignees": [
        "hirokisan"
      ],
      "participants": [
        "s-prosvirnin",
        "hirokisan"
      ],
      "comments_count": 2,
      "created_at": "2024-04-21T10:35:14Z",
      "closed_at": "2024-04-22T11:54:24Z"
    },
    {
      "url": "https://github.com/hirokisan/bybit/issues/160",
      "author": "biancheng347",
      "assignees": null,
      "participants": [
        "hirokisan",
        "biancheng347"
      ],
      "comments_count": 2,
      "created_at": "2024-01-25T09:18:51Z",
      "closed_at": "2024-01-25T10:01:03Z"
    }
  ]
}
```
