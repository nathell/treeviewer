#!/bin/sh

TAG="build-`date +%Y%m%d-%H%M%S`"
COMMIT_DESC="Build attempt `date`"

{

git stash push -k -u
git stash apply
git add -A
git commit -m "$COMMIT_DESC"
git tag -a -m "$COMMIT_DESC" "$TAG"
git reset --hard HEAD^
git stash pop

} >/dev/null

echo Created build tag: $TAG
