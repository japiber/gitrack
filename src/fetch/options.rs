// Warning!! Code generated automatically: this file must not be edited by hand

use std::process::Command;

use crate::wrap_command::FnOptionArg;

/// Fetch all remotes.
/// --all
pub fn all() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--all");
    })
}

/// Append ref names and object names of fetched refs to the existing contents of .git/FETCH_HEAD.
/// Without this option old data in .git/FETCH_HEAD will be overwritten.
/// -a, --append
pub fn append() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--append");
    })
}

/// Limit fetching to the specified number of commits from the tip of each remote branch history.
/// If fetching to a shallow repository created by git clone with --depth=<depth> option (see git-clone(1)), deepen or shorten the history to the specified number of commits.
/// Tags for the deepened commits are not fetched.
/// --depth=<depth>
pub fn depth(depth_arg: &str) -> FnOptionArg {
    let l_depth_arg = format!("--depth={}", depth_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_depth_arg.as_str());
    })
}

/// Similar to --depth, except it specifies the number of commits from the current shallow boundary instead of from the tip of each remote branch history.
/// --deepen=<depth>
pub fn deepen(depth_arg: &str) -> FnOptionArg {
    let l_depth_arg = format!("--deepen={}", depth_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_depth_arg.as_str());
    })
}

/// Deepen or shorten the history of a shallow repository to include all reachable commits after <date>.
/// --shallow-since=<date>
pub fn shallow_since(date_arg: &str) -> FnOptionArg {
    let l_date_arg = format!("--shallow-since={}", date_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_date_arg.as_str());
    })
}

/// Deepen or shorten the history of a shallow repository to exclude commits reachable from a specified remote branch or tag.
/// This option can be specified multiple times.
/// --shallow-exclude=<revision>
pub fn shallow_exclude(revision_arg: &str) -> FnOptionArg {
    let l_revision_arg = format!("--shallow-exclude={}", revision_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_revision_arg.as_str());
    })
}

/// If the source repository is complete, convert a shallow repository to a complete one, removing all the limitations imposed by shallow repositories.
/// If the source repository is shallow, fetch as much as possible so that the current repository has the same history as the source repository.
/// --unshallow
pub fn unshallow() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--unshallow");
    })
}

/// By default when fetching from a shallow repository, git fetch refuses refs that require updating .git/shallow.
/// This option updates .git/shallow and accept such refs.
/// --update-shallow
pub fn update_shallow() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--update-shallow");
    })
}

/// Show what would be done, without making any changes.
/// --dry-run
pub fn dry_run() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--dry-run");
    })
}

/// When git fetch is used with <rbranch>:<lbranch> refspec, it refuses to update the local branch <lbranch> unless the remote branch <rbranch> it fetches is a descendant of <lbranch>.
/// This option overrides that check.
/// -f, --force
pub fn force() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--force");
    })
}

/// Keep downloaded pack.
/// -k, --keep
pub fn keep() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--keep");
    })
}

/// Allow several <repository> and <group> arguments to be specified.
/// No <refspec>s may be specified.
/// --multiple
pub fn multiple() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--multiple");
    })
}

/// Before fetching, remove any remote-tracking references that no longer exist on the remote.
/// -p, --prune
pub fn prune() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--prune");
    })
}

/// By default, tags that point at objects that are downloaded from the remote repository are fetched and stored locally.
/// This option disables this automatic tag following.
/// The default behavior for a remote may be specified with the remote.<name>.tagOpt setting.
/// -n, --no-tags
pub fn no_tags() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-tags");
    })
}

/// When fetching refs listed on the command line, use the specified refspec (can be given more than once) to map the refs to remote-tracking branches, instead of the values of remote.*.fetch configuration variables for the remote repository.
/// --refmap=<refspec>
pub fn refmap(refspec_arg: &str) -> FnOptionArg {
    let l_refspec_arg = format!("--refmap={}", refspec_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_refspec_arg.as_str());
    })
}

/// Fetch all tags from the remote (i.e., fetch remote tags refs/tags/* into local tags with the same name), in addition to whatever else would otherwise be fetched.
/// -t, --tags
pub fn tags() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--tags");
    })
}

/// This option controls if and under what conditions new commits of populated submodules should be fetched too.
/// --recurse-submodules[=yes|on-demand|no]
pub fn recurse_submodules(value: &str) -> FnOptionArg {
    let l_value = if value.is_empty() {
        String::from("--recurse-submodules")
    } else {
        format!("--recurse-submodules={}", value)
    };
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_value.as_str());
    })
}

/// Number of parallel children to be used for fetching submodules.
/// Each will fetch from different submodules, such that fetching many submodules will be faster.
/// By default submodules will be fetched one at a time.
/// -j, --jobs=<n>
pub fn jobs(n_arg: &str) -> FnOptionArg {
    let l_n_arg = format!("--jobs={}", n_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_n_arg.as_str());
    })
}

/// Disable recursive fetching of submodules (this has the same effect as using the --recurse-submodules=no option).
/// --no-recurse-submodules
pub fn no_recurse_submodules() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-recurse-submodules");
    })
}

/// Prepend <path> to paths printed in informative messages.
/// --submodule-prefix=<path>
pub fn submodule_prefix(path_arg: &str) -> FnOptionArg {
    let l_path_arg = format!("--submodule-prefix={}", path_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_path_arg.as_str());
    })
}

/// This option is used internally to temporarily provide a non-negative default value for the --recurse-submodules option.
/// All other methods of configuring fetch’s submodule recursion (such as settings in gitmodules(5) and git-config(1)) override this option, as does specifying --[no-]recurse-submodules directly.
/// --recurse-submodules-default=[yes|on-demand]
pub fn recurse_submodules_default(value: &str) -> FnOptionArg {
    let l_value = if value.is_empty() {
        String::from("--recurse-submodules-default")
    } else {
        format!("--recurse-submodules-default={}", value)
    };
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_value.as_str());
    })
}

/// By default git fetch refuses to update the head which corresponds to the current branch.
/// This flag disables the check.
/// This is purely for the internal use for git pull to communicate with git fetch, and unless you are implementing your own Porcelain you are not supposed to use it.
/// -u, --update-head-ok
pub fn update_head_ok() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--update-head-ok");
    })
}

/// When given, and the repository to fetch from is handled by git fetch-pack, --exec=<upload-pack> is passed to the command to specify non-default path for the command run on the other end.
/// --upload-pack <upload-pack>
pub fn upload_pack(upload_pack_arg: &str) -> FnOptionArg {
    let l_upload_pack_arg = String::from(upload_pack_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--upload-pack");
        cmd.arg(l_upload_pack_arg.as_str());
    })
}

/// Pass --quiet to git-fetch-pack and silence any other internally used git commands.
/// Progress is not reported to the standard error stream.
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

/// Progress status is reported on the standard error stream by default when it is attached to a terminal, unless -q is specified.
/// This flag forces progress status even if the standard error stream is not directed to a terminal.
/// --progress
pub fn progress() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--progress");
    })
}

/// Use IPv4 addresses only, ignoring IPv6 addresses.
/// -4, --ipv4
pub fn ipv4() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--ipv4");
    })
}

/// Use IPv6 addresses only, ignoring IPv4 addresses.
/// -6, --ipv6
pub fn ipv6() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--ipv6");
    })
}