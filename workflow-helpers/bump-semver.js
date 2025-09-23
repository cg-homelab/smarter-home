
import * as semver from 'semver';

module.exports = async ({ github, context, core }) => {
  try {
    const currentVersion = process.env.INPUT_TAG;
    const bumpLevel = process.env.BUMP_LEVEL || 'patch';

    const newVersion = await bumpSemver(currentVersion, bumpLevel);
    core.setOutput('new_version', newVersion);
  } catch (e) {
    core.error(e);
    core.setFailed(e.message);
  }
}

async function bumpSemver(
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
