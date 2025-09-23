
// @ts-check
/** @param {import('@actions/github-script').AsyncFunctionArguments} git */
/** @param {import('semver')} semver */
export default async function run(git, semver) {
  git.core.debug("Running something at the moment");
  try {
    const currentVersion = process.env.INPUT_TAG;
    const bumpLevel = process.env.BUMP_LEVEL || 'patch';

    const newVersion = await bumpSemver(semver, currentVersion, bumpLevel);
    git.core.setOutput('new_version', newVersion);
  } catch (e) {
    git.core.error(e);
    git.core.setFailed(e.message);
  }
}

async function bumpSemver(
  semver,
  currentVersion,
  bumpLevel
) {
  if (!semver.valid(currentVersion)) {
    throw new Error(`${currentVersion} is not a valid semver`);
  }

  if (!isReleaseType(bumpLevel)) {
    throw new Error(
      `${bumpLevel} is not supported. {major, premajor, minor, preminor, patch, prepatch, prerelease} is available.`
    );
  }

  // https://semver.org/#is-v123-a-semantic-version
  // If the current version has 'v' prefix (e.g., v1.2.3), keep the prefix in the new version too.
  const hasVPrefix = currentVersion.startsWith('v');

  const bumpedVersion = semver.inc(currentVersion, bumpLevel);

  let newVersion = bumpedVersion;
  if (hasVPrefix) {
    newVersion = `v${newVersion}`;
  }

  return newVersion;
}

function isReleaseType(s) {
  return [
    'major',
    'premajor',
    'minor',
    'preminor',
    'patch',
    'prepatch',
    'prerelease'
  ].includes(s);
}
