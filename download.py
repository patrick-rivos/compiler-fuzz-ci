#! /bin/python3
# Download the Fuzzer's output artifacts from CI

import argparse
import os
from typing import List, Set
from zipfile import ZipFile
# pygithub
from github import Auth, Github
import requests
import shutil


def parse_arguments():
    """Parse command line arguments"""
    parser = argparse.ArgumentParser(description="Download single log artifact")
    parser.add_argument(
        "-name",
        type=str,
        help="Name of the artifact to download.",
        default="discoveries.zip"
    )
    parser.add_argument(
        "-repo", type=str, help="Github repo to search/download in",
        default="patrick-rivos/compiler-fuzz-ci"
    )
    parser.add_argument(
        "-token",
        required=True,
        type=str,
        help="Github access token.",
    )
    parser.add_argument(
        "-outdir", required=True, type=str, help="Output dir to put downloaded files"
    )
    return parser.parse_args()


def search_for_artifact(
    artifact_name: str, repo_name: str, token: str, github: "Github | None" = None
) -> "List[str] | None":
    """
    Search for the given artifact.
    Returns the list of artifact ids or None if the artifact was not found.
    """
    if github is None:
        auth = Auth.Token(token)
        github = Github(auth=auth)

    repo = github.get_repo(repo_name)

    artifacts = repo.get_artifacts(artifact_name).get_page(0)
    if len(artifacts) != 0:
        [print(artifact.size_in_bytes) for artifact in artifacts]
        return [str(artifact.id) for artifact in artifacts if artifact.size_in_bytes < 2 ** 30] # filter by 1 gb

    return None


def download_artifact(artifact_name: str, artifact_id: str, token: str, repo: str) -> str:
    """
    Uses GitHub api endpoint to download the given artifact into ./temp/.
    Returns the path of the downloaded zip
    """
    params = {
        "Accept": "application/vnd.github+json",
        "Authorization": f"token {token}",
        "X-Github-Api-Version": "2022-11-28",
    }
    response = requests.get(
        f"https://api.github.com/repos/{repo}/actions/artifacts/{artifact_id}/zip",
        headers=params,
        timeout=15 * 60,  # 15 minute timeout
    )
    print(f"download for {artifact_name}: {response.status_code}")

    artifact_zip_name = artifact_name.replace(".log", ".zip")
    artifact_zip = f"./temp/{artifact_zip_name}"

    with open(artifact_zip, "wb") as artifact:
        artifact.write(response.content)

    return artifact_zip


def extract_artifact(
    artifact_name: str, artifact_zip: str, outdir: str,
    artifact_id: str, prefix: str,
):
    """
    Extracts a given artifact into the outdir.
    A bit hacky/tailored to the CI's zip structure.
    """
    os.makedirs('./temp', exist_ok=True)

    with ZipFile(artifact_zip, "r") as zf:
        zf.extractall(path="./temp/first/")

    with ZipFile(f"./temp/first/{artifact_name}", "r") as zf:
        zf.extractall(path="./temp/second/")

    shutil.move(
        './temp/second/out',
        f'{outdir}/{prefix}-{artifact_id}'
    )


def main():
    args = parse_arguments()

    existing_artifacts: Set[str] = set(os.listdir(args.outdir))

    prefixes = [
        "gcc-14",
        "gcc-15",
        "gcc-master",
        "gcc-arm",
        "llvm-main",
    ]

    for prefix in prefixes:
        artifact_name = f"{prefixes}-{args.name}"
        artifact_ids = search_for_artifact(artifact_name, args.repo, args.token)
        if artifact_ids is None:
            raise ValueError(f"Could not find artifact {artifact_name} in {args.repo}")

        new_artifact_ids = [a_id for a_id in artifact_ids if a_id not in existing_artifacts]

        if len(new_artifact_ids) == 0:
            print(f"No new artifacts found ({len(existing_artifacts)}/{len(artifact_ids)} in {args.outdir})")

        for artifact_id in new_artifact_ids:
            print(f"Processing {artifact_id}")
            artifact_zip = download_artifact(artifact_name, artifact_id, args.token, args.repo)
            print(os.path.getsize(artifact_zip))
            extract_artifact(artifact_name, artifact_zip, args.outdir, artifact_id, prefix)

if __name__ == "__main__":
    main()
