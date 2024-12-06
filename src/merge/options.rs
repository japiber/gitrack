// Warning!! Code generated automatically: this file must not be edited by hand

use std::process::Command;

use crate::wrap_command::FnOptionArg;

/// Perform the merge and commit the result.
/// This option can be used to override --no-commit.
/// --commit, --no-commit
pub fn commit() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--commit");
    })
}

/// With --no-commit perform the merge but pretend the merge failed and do not autocommit, to give the user a chance to inspect and further tweak the merge result before committing.
/// --commit, --no-commit
pub fn no_commit() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-commit");
    })
}

/// Invoke an editor before committing successful mechanical merge to further edit the auto-generated merge message, so that the user can explain and justify the merge.
/// The --no-edit option can be used to accept the auto-generated message (this is generally discouraged).
/// The --edit (or -e) option is still useful if you are giving a draft message with the -m option from the command line and want to edit it in the editor.
/// Older scripts may depend on the historical behaviour of not allowing the user to edit the merge log message.
/// They will see an editor opened when they run git merge.
/// To make it easier to adjust such scripts to the updated behaviour, the environment variable GIT_MERGE_AUTOEDIT can be set to no at the beginning of them.
/// --edit, -e, --no-edit
pub fn edit() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--edit");
    })
}

/// Invoke an editor before committing successful mechanical merge to further edit the auto-generated merge message, so that the user can explain and justify the merge.
/// The --no-edit option can be used to accept the auto-generated message (this is generally discouraged).
/// The --edit (or -e) option is still useful if you are giving a draft message with the -m option from the command line and want to edit it in the editor.
/// Older scripts may depend on the historical behaviour of not allowing the user to edit the merge log message.
/// They will see an editor opened when they run git merge.
/// To make it easier to adjust such scripts to the updated behaviour, the environment variable GIT_MERGE_AUTOEDIT can be set to no at the beginning of them.
/// --edit, -e, --no-edit
pub fn no_edit() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-edit");
    })
}

/// When the merge resolves as a fast-forward, only update the branch pointer, without creating a merge commit.
/// This is the default behavior.
/// --ff
pub fn ff() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--ff");
    })
}

/// Create a merge commit even when the merge resolves as a fast-forward.
/// This is the default behaviour when merging an annotated (and possibly signed) tag.
/// --no-ff
pub fn no_ff() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-ff");
    })
}

/// Refuse to merge and exit with a non-zero status unless the current HEAD is already up-to-date or the merge can be resolved as a fast-forward.
/// --ff-only
pub fn ff_only() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--ff-only");
    })
}

/// In addition to branch names, populate the log message with one-line descriptions from at most <n> actual commits that are being merged.
/// See also git-fmt-merge-msg(1).
/// With --no-log do not list one-line descriptions from the actual commits being merged.
/// --log[=<n>], --no-log
pub fn log(n_arg: &str) -> FnOptionArg {
    let l_n_arg = format!("--log={}", n_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_n_arg.as_str());
    })
}

/// In addition to branch names, populate the log message with one-line descriptions from at most <n> actual commits that are being merged.
/// See also git-fmt-merge-msg(1).
/// With --no-log do not list one-line descriptions from the actual commits being merged.
/// --log[=<n>], --no-log
pub fn no_log() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-log");
    })
}

/// Show a diffstat at the end of the merge.
/// The diffstat is also controlled by the configuration option merge.stat.
/// With -n or --no-stat do not show a diffstat at the end of the merge.
/// --stat, -n, --no-stat
pub fn stat() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--stat");
    })
}

/// Show a diffstat at the end of the merge.
/// The diffstat is also controlled by the configuration option merge.stat.
/// With -n or --no-stat do not show a diffstat at the end of the merge.
/// --stat, -n, --no-stat
pub fn no_stat() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-stat");
    })
}

/// Produce the working tree and index state as if a real merge happened (except for the merge information), but do not actually make a commit, move the HEAD, or record $GIT_DIR/MERGE_HEAD (to cause the next git commit command to create a merge commit).
/// This allows you to create a single commit on top of the current branch whose effect is the same as merging another branch (or more in case of an octopus).
/// With --no-squash perform the merge and commit the result.
/// This option can be used to override --squash.
/// --squash, --no-squash
pub fn squash() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--squash");
    })
}

/// Produce the working tree and index state as if a real merge happened (except for the merge information), but do not actually make a commit, move the HEAD, or record $GIT_DIR/MERGE_HEAD (to cause the next git commit command to create a merge commit).
/// This allows you to create a single commit on top of the current branch whose effect is the same as merging another branch (or more in case of an octopus).
/// With --no-squash perform the merge and commit the result.
/// This option can be used to override --squash.
/// --squash, --no-squash
pub fn no_squash() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-squash");
    })
}

/// Use the given merge strategy; can be supplied more than once to specify them in the order they should be tried.
/// If there is no -s option, a built-in list of strategies is used instead (git merge-recursive when merging a single head, git merge-octopus otherwise).
/// -s <strategy>, --strategy=<strategy>
pub fn strategy(strategy_arg: &str) -> FnOptionArg {
    let l_strategy_arg = format!("--strategy={}", strategy_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_strategy_arg.as_str());
    })
}

/// Pass merge strategy specific option through to the merge strategy.
/// -X <option>, --strategy-option=<option>
pub fn strategy_option(option_arg: &str) -> FnOptionArg {
    let l_option_arg = format!("--strategy-option={}", option_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_option_arg.as_str());
    })
}

/// Verify that the tip commit of the side branch being merged is signed with a valid key, i.e. a key that has a valid uid: in the default trust model, this means the signing key has been signed by a trusted key.
/// If the tip commit of the side branch is not signed with a valid key, the merge is aborted.
/// --verify-signatures, --no-verify-signatures
pub fn verify_signatures() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--verify-signatures");
    })
}

/// Verify that the tip commit of the side branch being merged is signed with a valid key, i.e. a key that has a valid uid: in the default trust model, this means the signing key has been signed by a trusted key.
/// If the tip commit of the side branch is not signed with a valid key, the merge is aborted.
/// --verify-signatures, --no-verify-signatures
pub fn no_verify_signatures() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-verify-signatures");
    })
}

/// Synonyms to --stat and --no-stat; these are deprecated and will be removed in the future.
/// --summary, --no-summary
pub fn summary() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--summary");
    })
}

/// Synonyms to --stat and --no-stat; these are deprecated and will be removed in the future.
/// --summary, --no-summary
pub fn no_summary() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-summary");
    })
}

/// Operate quietly.
/// Implies --no-progress.
/// -q, --quiet
pub fn quiet() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--quiet");
    })
}

/// Be verbose.
/// -v, --verbose
pub fn verbose() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--verbose");
    })
}

/// Turn progress on/off explicitly.
/// If neither is specified, progress is shown if standard error is connected to a terminal.
/// Note that not all merge strategies may support progress reporting.
/// --progress, --no-progress
pub fn progress() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--progress");
    })
}

/// Turn progress on/off explicitly.
/// If neither is specified, progress is shown if standard error is connected to a terminal.
/// Note that not all merge strategies may support progress reporting.
/// --progress, --no-progress
pub fn no_progress() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-progress");
    })
}

/// By default, git merge command refuses to merge histories that do not share a common ancestor.
/// This option can be used to override this safety when merging histories of two projects that started their lives independently.
/// As that is a very rare occasion, no configuration variable to enable this by default exists and will not be added.
/// --allow-unrelated-histories
pub fn allow_unrelated_histories() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--allow-unrelated-histories");
    })
}

/// GPG-sign the resulting merge commit.
/// The keyid argument is optional and defaults to the committer identity; if specified, it must be stuck to the option without a space.
/// -S[<keyid>], --gpg-sign[=<keyid>]
pub fn gpg_sign(keyid_arg: &str) -> FnOptionArg {
    let l_keyid_arg = format!("--gpg-sign={}", keyid_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_keyid_arg.as_str());
    })
}

/// Set the commit message to be used for the merge commit (in case one is created).
/// If --log is specified, a shortlog of the commits being merged will be appended to the specified message.
/// The git fmt-merge-msg command can be used to give a good default for automated git merge invocations.
/// The automated message can include the branch description.
/// -m <msg>
pub fn m(msg_arg: &str) -> FnOptionArg {
    let l_msg_arg = String::from(msg_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("-m");
        cmd.arg(l_msg_arg.as_str());
    })
}

/// Allow the rerere mechanism to update the index with the result of auto-conflict resolution if possible.
/// --[no-]rerere-autoupdate
pub fn rerere_autoupdate() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--rerere-autoupdate");
    })
}

/// Allow the rerere mechanism to update the index with the result of auto-conflict resolution if possible.
/// --[no-]rerere-autoupdate
pub fn no_rerere_autoupdate() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-rerere-autoupdate");
    })
}

/// Abort the current conflict resolution process, and try to reconstruct the pre-merge state.
/// If there were uncommitted worktree changes present when the merge started, git merge --abort will in some cases be unable to reconstruct these changes.
/// It is therefore recommended to always commit or stash your changes before running git merge.
/// git merge --abort is equivalent to git reset --merge when MERGE_HEAD is present.
/// --abort
pub fn abort() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--abort");
    })
}

/// After a git merge stops due to conflicts you can conclude the merge by running git merge --continue (see 'HOW TO RESOLVE CONFLICTS' section below).
/// --continue
pub fn continue_merge() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--continue");
    })
}