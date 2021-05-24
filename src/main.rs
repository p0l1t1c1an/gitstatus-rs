use git2::Error;
use git2::Repository;
use git2::Status;
use git2::StatusOptions;

/*
 * Icons for status to be printed
 * (similar to romkatv/gitstatus)
 *
 *  @ = At this commit
 *  | = behind of remote
 *  & = ahead of remote
 *  ~ = merge conflicts
 *  * = stashes
 *  + = staged changes
 *  ! = unstaged changes
 *  ? = new/untracked files
 */

const COMMIT_ICON: char = '@';
const BEHIND_ICON: char = '|';
const AHEAD_ICON: char = '&';
const MERGE_ICON: char = '~';
const STASH_ICON: char = '*';
const STAGED_ICON: char = '+';
const UNSTAGED_ICON: char = '!';
const UNTRACKED_ICON: char = '?';

fn main() -> Result<(), Error> {
    if let Ok(mut repo) = Repository::discover(".") {
        let mut prompt = String::new();

        if let Ok(head_ref) = repo.head() {
            let head_commit = head_ref.peel_to_commit()?;
            if head_ref.is_branch() {
                prompt += &String::from_utf8_lossy(head_ref.shorthand_bytes());

                if let Some(head_name) = head_ref.name() {
                    if let Ok(orig_name) = repo.branch_upstream_name(&head_name) {
                        if let Ok(orig_ref) =
                            repo.find_reference(&String::from_utf8_lossy(&orig_name))
                        {
                            let orig_commit = orig_ref.peel_to_commit()?;
                            let (ahead, behind) =
                                repo.graph_ahead_behind(head_commit.id(), orig_commit.id())?;

                            if behind > 0 {
                                prompt += &format!(" {}{}", BEHIND_ICON, behind);
                            }
                            if ahead > 0 {
                                prompt += &format!(" {}{}", AHEAD_ICON, ahead);
                            }
                        }
                    }
                }
            } else {
                let mut hash = format!("{}{}", COMMIT_ICON, head_commit.id());
                hash.truncate(7);
                prompt += &hash;
            }
        }

        let mut stashed = 0;
        repo.stash_foreach(|_, _, _| -> bool {
            stashed += 1;
            true
        })?;

        let (mut conflicted, mut untracked) = (0, 0);
        let (mut staged, mut unstaged) = (0, 0);

        // Bits are ORed together
        let staged_bits = Status::INDEX_MODIFIED
            | Status::INDEX_DELETED
            | Status::INDEX_TYPECHANGE
            | Status::INDEX_RENAMED
            | Status::INDEX_NEW;

        let unstaged_bits =
            Status::WT_MODIFIED | Status::WT_DELETED | Status::WT_TYPECHANGE | Status::WT_RENAMED;

        let mut options = StatusOptions::new();
        options
            .include_untracked(true)
            .recurse_untracked_dirs(true)
            .renames_head_to_index(true);

        let statuses = repo.statuses(Some(&mut options))?;
        for entry in statuses.iter() {
            let s = entry.status();
            if s.intersects(Status::CONFLICTED) {
                conflicted += 1;
            }
            if s.intersects(Status::WT_NEW) {
                untracked += 1;
            }
            if s.intersects(staged_bits) {
                staged += 1;
            }
            if s.intersects(unstaged_bits) {
                unstaged += 1;
            }
        }

        if conflicted > 0 {
            prompt += &format!(" {}{}", MERGE_ICON, conflicted);
        }

        if stashed > 0 {
            prompt += &format!(" {}{}", STASH_ICON, stashed);
        }

        if staged > 0 {
            prompt += &format!(" {}{}", STAGED_ICON, staged);
        }

        if unstaged > 0 {
            prompt += &format!(" {}{}", UNSTAGED_ICON, unstaged);
        }

        if untracked > 0 {
            prompt += &format!(" {}{}", UNTRACKED_ICON, &untracked);
        }

        println!("{}", prompt.trim());
    }

    Ok(())
}
