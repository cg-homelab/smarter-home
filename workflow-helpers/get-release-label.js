module.exports = async ({ github, context, core }) => {
  try {
    // Get labels for pull_request
    const labels = await github.rest.issues.listLabelsOnIssue({
      owner: context.repo.owner,
      repo: context.repo.repo,
      issue_number: context.issue.number,
    }).map(label => label.name);

    let level = null;
    if (labels.includes('release/major')) {
      level = 'major';
    } else if (labels.includes('release/minor')) {
      level = 'minor';
    } else if (labels.includes('release/patch')) {
      level = 'patch';
    } else {
      level = 'patch';
    }
    core.setOutput('level', level);
  } catch (e) {
    core.error(e);
    core.setFailed(e.message);
  }
}
