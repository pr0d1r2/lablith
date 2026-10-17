# shellcheck shell=bash
#
# open-mr.sh BRANCH TITLE BODY [PATH...]
#
# Commit PATH (default: every change) on a new BRANCH, push it and open a
# merge request titled TITLE into the default branch (kinds `update-pins`
# and `upstream`). The token is the CI/CD variable LABLITH_TOKEN_NAME
# names (default LABLITH_TOKEN): a project access token that may push a
# branch and open a merge request (src/kind:V54). It travels in headers
# only and is never printed (src/kind:V13). The GitLab API is reached
# with curl, so no extra CLI joins the dev shell (src/script:V26).
#
# Idempotent (src/bump:V33): a branch already on the project means an
# earlier run opened its merge request, so this exits 0 and opens no
# second one. Nothing staged also exits 0.
set -euo pipefail

if [ "$#" -lt 3 ]; then
  echo "usage: open-mr.sh BRANCH TITLE BODY [PATH...]" >&2
  exit 2
fi
branch="$1"
title="$2"
body="$3"
shift 3

name="${LABLITH_TOKEN_NAME:-LABLITH_TOKEN}"
token="${!name:-}"
if [ -z "$token" ]; then
  echo "open-mr: CI/CD variable $name is not set; add it as a masked, protected project access token" >&2
  exit 1
fi
remote="${CI_SERVER_URL}/${CI_PROJECT_PATH}.git"
auth="Authorization: Basic $(printf 'oauth2:%s' "$token" | base64 | tr -d '\n')"
authed() { git -c "http.extraHeader=$auth" "$@"; }

if authed ls-remote --exit-code --heads "$remote" "$branch" >/dev/null 2>&1; then
  echo "open-mr: branch $branch already exists, nothing to do"
  exit 0
fi

git config user.name "lablith"
git config user.email "lablith@users.noreply.invalid"
git switch -c "$branch"
if [ "$#" -eq 0 ]; then
  git add -A
else
  git add -- "$@"
fi
if git diff --cached --quiet; then
  echo "open-mr: nothing changed, no merge request"
  exit 0
fi
git commit -m "$title" -m "$body"
authed push "$remote" "HEAD:refs/heads/$branch"
curl -fsS -o /dev/null -X POST -H "PRIVATE-TOKEN: $token" \
  --data-urlencode "source_branch=$branch" \
  --data-urlencode "target_branch=$CI_DEFAULT_BRANCH" \
  --data-urlencode "title=$title" \
  --data-urlencode "description=$body" \
  --data "remove_source_branch=true" \
  "$CI_API_V4_URL/projects/$CI_PROJECT_ID/merge_requests"
echo "open-mr: opened a merge request from $branch"
