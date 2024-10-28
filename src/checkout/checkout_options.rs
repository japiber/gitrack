use crate::command_executor::{CommandExecutor, CommandOption};

/// Quiet, suppress feedback messages.
/// -q, --quiet
pub fn quiet_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--quiet"))
}

/// Progress status is reported on the standard error stream by default when it is attached to a terminal, unless --quiet is specified.
/// This flag enables progress reporting even if not attached to a terminal, regardless of --quiet.
/// --progress
pub fn progress_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--progress"))
}

/// Progress status is reported on the standard error stream by default when it is attached to a terminal, unless --quiet is specified.
/// This flag enables progress reporting even if not attached to a terminal, regardless of --quiet.
/// --no-progress
pub fn no_progress_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--no-progress"))
}

/// When switching branches, proceed even if the index or the working tree differs from HEAD.
/// This is used to throw away local changes.
/// When checking out paths from the index, do not fail upon unmerged entries; instead, unmerged entries are ignored.
/// -f, --force
pub fn force_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--force"))
}

/// When checking out paths from the index, check out stage #2 (ours) or #3 (theirs) for unmerged paths.
/// --ours, --theirs
pub fn ours_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--ours"))
}

/// When checking out paths from the index, check out stage #2 (ours) or #3 (theirs) for unmerged paths.
/// --ours, --theirs
pub fn theirs_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--theirs"))
}

/// Create a new branch named <new_branch> and start it at <start_point>; see git-branch(1) for details.
/// -b [new_branch]
pub fn b_option(new_branch_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
         g.add_option("-b");
        if !new_branch_arg.is_empty() {
            g.add_option(new_branch_arg);
        }
    })
}


/// Creates the branch <new_branch> and start it at <start_point>; if it already exists, then reset it to <start_point>.
/// This is equivalent to running 'git branch' with '-f'; see git-branch(1) for details.
/// -B [new_branch]
pub fn b_option(new_branch_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
         g.add_option("-B");
        if !new_branch_arg.is_empty() {
            g.add_option(new_branch_arg);
        }
    })
}


/// When creating a new branch, set up 'upstream' configuration.
/// See '--track' in git-branch(1) for details.
/// -t, --track
pub fn track_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--track"))
}

/// Do not set up 'upstream' configuration, even if the branch.autoSetupMerge configuration variable is true.
/// --no-track
pub fn no_track_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--no-track"))
}

/// Create the new branch’s reflog; see git-branch(1) for details.
/// -l
pub fn l_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("-l"))
}

/// Rather than checking out a branch to work on it, check out a commit for inspection and discardable experiments.
/// This is the default behavior of 'git checkout <commit>' when <commit> is not a branch name.
/// See the 'DETACHED HEAD' section below for details.
/// --detach
pub fn detach_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--detach"))
}

/// Create a new orphan branch, named <new_branch>, started from <start_point> and switch to it.
/// The first commit made on this new branch will have no parents and it will be the root of a new history totally disconnected from all the other branches and commits.
/// --orphan <new_branch>
pub fn orphan_option(new_branch_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--orphan");
        g.add_option(new_branch_arg);
    })
}


/// In sparse checkout mode, git checkout -- <paths> would update only entries matched by <paths> and sparse patterns in $GIT_DIR/info/sparse-checkout.
/// This option ignores the sparse patterns and adds back any files in <paths>.
/// --ignore-skip-worktree-bits
pub fn ignore_skip_worktree_bits_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--ignore-skip-worktree-bits"))
}

/// When switching branches, if you have local modifications to one or more files that are different between the current branch and the branch to which you are switching, the command refuses to switch branches in order to preserve your modifications in context.
/// However, with this option, a three-way merge between the current branch, your working tree contents, and the new branch is done, and you will be on the new branch.
/// -m, --merge
pub fn merge_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--merge"))
}

/// The same as --merge option above, but changes the way the conflicting hunks are presented, overriding the merge.conflictStyle configuration variable.
/// Possible values are 'merge' (default) and 'diff3' (in addition to what is shown by 'merge' style, shows the original contents).
/// --conflict=<style>
pub fn conflict_option(style_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--conflict={}", style_arg)))
}

/// Interactively select hunks in the difference between the <tree-ish> (or the index, if unspecified) and the working tree.
/// The chosen hunks are then applied in reverse to the working tree (and if a <tree-ish> was specified, the index).
/// -p, --patch
pub fn patch_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--patch"))
}

/// git checkout refuses when the wanted ref is already checked out by another worktree.
/// This option makes it check the ref out anyway.
/// In other words, the ref can be held by more than one worktree.
/// --ignore-other-worktrees
pub fn ignore_other_worktrees_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--ignore-other-worktrees"))
}

/// Using --recurse-submodules will update the content of all initialized submodules according to the commit recorded in the superproject.
/// If local modifications in a submodule would be overwritten the checkout will fail unless -f is used.
/// If nothing (or --no-recurse-submodules) is used, the work trees of submodules will not be updated.
/// --recurse-submodules
pub fn recurse_submodules_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--recurse-submodules"))
}

/// Using --recurse-submodules will update the content of all initialized submodules according to the commit recorded in the superproject.
/// If local modifications in a submodule would be overwritten the checkout will fail unless -f is used.
/// If nothing (or --no-recurse-submodules) is used, the work trees of submodules will not be updated.
/// --no-recurse-submodules
pub fn no_recurse_submodules_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--no-recurse-submodules"))
}