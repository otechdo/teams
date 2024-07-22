# Committer

A program to manage a git repository.

- It's purpose tools to:
  - Commit message
  - Generate change log
  - Create && finish features
  - Manage branches
  - Manages tags
  - Display tags
  - See git logs
  - Add && send modifications
  - Create a branch with stash
  - See branches with the latest commit
  - remove cargo dependencies
  - Use the `hunspell` dictionary for commit message
  - Install cargo project locally
  - No commit hooks necessary
  - Run test
  - Run clippy linter
  - Run `cargo fmt` automaticaly to format code source

> it's require `hunspell` command available and `en_US` dict installed.

## Installation depedencies

### Fedora

```bash
sudo dnf install hunspell hunspell-en_US rustup git
```

### Debian

```bash
sudo apt install hunspell hunspell-en_US rustup git python-pip
```

### Silverblue

```bash
rpm-ostree install hunspell hunspell-en_US rustup git python-pip
```

### ArchLinux

```bash
sudo pacman -S hunspell hunspell-en_US rustup git python-pip
```

### Cargo initialisation

```bash
rustup default stable
```

### Git fame

```bash
pip install git-fame
```

### Commiter installation

```bash
cargo install cargo-watch zuu commiter
```

### Commit Message Format

```git
<type>(<scope>): <short summary>

[Description with more details]

[Why changes details]

[Footer]
```

### Commit Types

| Category                          | Commit Type                | Mnemonic                                                                                           | Description                                                    | Example                                                                                 |
| :-------------------------------- | :------------------------- | :------------------------------------------------------------------------------------------------- | :------------------------------------------------------------- | :-------------------------------------------------------------------------------------- |
| **Core Changes**                  | Star                       | Shiny Technology Added or Refined                                                                  | New feature or enhancement                                     | `Star(Auth): Implement two-factor authentication`                                       |
|                                   | Comet                      | Code or Module Error Terminated                                                                    | Bug fix or error resolution                                    | `Comet(UI): Fix responsive layout issue on mobile devices`                              |
|                                   | Nebula                     | New Efficient Better Understandable Logic Achieved                                                 | Code refactoring                                               | `Nebula(Backend): Refactor user management module for improved maintainability`         |
|                                   | Pulsar                     | Powerful Upgrade, Less Sluggish, Agile Response                                                    | Performance improvement                                        | `Pulsar(Database): Optimize queries for faster response times`                          |
|                                   | Quasar                     | Quick Adjustments for Superior Accuracy and Readability                                            | Documentation or clarity improvement                           | `Quasar(API): Update documentation with new endpoint parameters`                        |
| **Maintenance & Infrastructure**  | Asteroid Belt              | Adjustments, Sweeps, Tidy-ups, Elimination, Reordering of Items, Decrease Bloat                    | Code cleanup and maintenance                                   | `Asteroid Belt: Remove unused CSS and optimize images`                                  |
|                                   | Solar Flare                | Securing Our Logic Against Regressions, Failures, and Latencies Actively, Rigorously Ensured       | Adding or updating tests (unit, integration, end-to-end).      | `Solar Flare(Payments): Add unit tests for payment processing module`                   |
|                                   | Dwarf Planet               | Details Warranted Attention, Refined Further, Polished Little Aspects Neatly Enhanced Tiny         | Minor but essential updates or fixes.                          | `Dwarf Planet: Update project dependencies to latest versions`                          |
|                                   | Terraform                  | Technology Engineering Resources Readily Automated, Foundation of Reliable Management              | Infrastructure changes                                         | `Terraform(AWS): Provision new EC2 instance for staging environment`                    |
| **Project Events**                | Black Hole                 | Big Legacy Aspects Consumed, Killing Heavy, Old Loads Entirely                                     | Removing large chunks of code or features                      | `Black Hole: Remove deprecated user profile module`                                     |
|                                   | Wormhole                   | Weaving or Reconnecting Modules, Hitching onto Linked Elements                                     | Merging branches or connecting code parts                      | `Wormhole: Merge feature/new-dashboard into develop branch`                             |
|                                   | Big Bang                   | Birth of Initial Greatness, Beginning All New Growth                                               | Initial commit of a project or major feature                   | `Big Bang: Initial project setup and scaffolding`                                       |
|                                   | Launch                     | Lifting Application Upward, New Code Entering Production                                           | Deploying to production or releasing a version                 | `Launch(v1.2): Release new version with user profile customization`                     |
| **Communication & Collaboration** | Lightspeed                 | Lightening Speed Enhancements                                                                      | Significant performance improvements                           | `Lightspeed(Frontend): Implement lazy loading for images`                               |
|                                   | Mission Control            | Managing Changes, Issues, Scope, Teamwork, and Release On Time                                     | Project management changes                                     | `Mission Control: Update project roadmap and assign tasks for Q3`                       |
|                                   | Spacewalk                  | Swift Work Above Limits, Keeping All Systems Extra Safe                                            | Urgent hotfixes or critical production updates.                | `Spacewalk(Security): Patch critical vulnerability in authentication module`            |
|                                   | Moon Landing               | Major Leaps Over Night, New Doors and Incredible Achievements                                      | Completing major milestones or goals                           | `Moon Landing: Successfully launch beta version to select users`                        |
|                                   | First Contact              | Forge Initial Connections, Open New Territories                                                    | Establishing initial connections or integrations               | `First Contact(API): Integrate with new payment provider's API`                         |
|                                   | Interstellar Communication | Informing, Sharing, Teaching, Educating, & Learning Lucidly & Clearly                              | Improving documentation or communication                       | `Interstellar Communication: Update wiki with troubleshooting guide for common errors`  |
| **Celestial Events**              | Solar Eclipse              | Sun Escapes, Legacy Code Lurks                                                                     | Temporarily masking functionality.                             | `Solar Eclipse(Feature): Temporarily disable new onboarding flow for testing`           |
|                                   | Supernova                  | Sudden Unbelievable Performance Revolution, New Version Arrives                                    | Major, transformative change or improvement.                   | `Supernova(Architecture): Migrate to microservices architecture`                        |
|                                   | Meteor Shower              | Many Edits, Tiny Overall Result, Overhaul Routines                                                 | Series of small changes or fixes.                    Secondary | Lunar Eclipse                                                                           |
|                                   | Cosmic Dawn                | Creating Original, Simple, Minimal Initial Draft                                                   | Initial implementation of a feature.                           | `Cosmic Dawn(Search): Initial implementation of basic search functionality`             |
|                                   | Solar Storm                | Sudden Transformations Occur Rapidly, Modifications                                                | Rapid, impactful changes.                                      | `Solar Storm(Refactor): Overhaul data processing pipeline for improved performance`     |
|                                   | Lunar Transit              | Little Update, Now Adjustments Require Testing                                                     | Minor, temporary change.                                       | `Lunar Transit(Config): Temporarily adjust logging level for debugging`                 |
|                                   | Perihelion                 | Perfect Ending, Refined, Improved, High Efficiency, Low Obstacles, Near Goal                       | Significant milestone or feature completion.                   | `Perihelion: Successfully complete user acceptance testing for new dashboard`           |
|                                   | Aphelion                   | Away From Perfection, High Effort, Long Overhaul, Intense Overhaul, Obstacles                      | Refactor, dependency update, or architecture change.           | `Aphelion: Upgrade to React 18 and refactor components`                                 |
| **Celestial Objects**             | White Dwarf                | Writing, Improving, Detailed Documentation For All                                                 | Improving code comments or documentation                       | `White Dwarf(API): Add detailed documentation for new endpoints`                        |
|                                   | Red Giant                  | Refactoring, Enhancing, Growing, Increasing, Adding New Things                                     | Expanding a feature or functionality                           | `Red Giant(Payments): Add support for Apple Pay and Google Pay`                         |
|                                   | Neutron Star               | New Efficient Utility, Tweaks, Robust Optimization, Nimble Solution                                | Optimizing code for performance                                | `Neutron Star(Search): Optimize search algorithm for faster results`                    |
|                                   | Binary Star                | Bringing In New And Revised, Yielding Integrated Results                                           | Merging features or components                                 | `Binary Star: Merge user authentication and authorization modules`                      |
|                                   | Brown Dwarf                | Barely Developed, Requires Work, Ongoing Development For Future                                    | Undeveloped feature with potential                             | `Brown Dwarf(Social): Initial prototype for social sharing feature`                     |
|                                   | Quark Star                 | Questionable, Unstable, Anticipated Results, Risky, Keen Experiment                                | Experimental or speculative change                             | `Quark Star(AI): Experiment with integrating GPT-3 for content generation`              |
|                                   | Rogue Planet               | Refactoring Or Generating Operations, Unique Path, Leaping Ahead                                   | Independent change unrelated to the main codebase              | `Rogue Planet: Create standalone script for data migration`                             |
|                                   | Stellar Nursery            | Starting To Enhance, Laying Layers, Launching New Requirements                                     | Creating new components                                        | `Stellar Nursery(UI): Add new component library for design system`                      |
|                                   | Planetary Nebula           | Pruning, Leaving, Abandoning, Nostalgic Era, Totally Removed                                       | Removal or deprecation of a component                          | `Planetary Nebula: Remove legacy image carousel component`                              |
|                                   | Globular Cluster           | Gathering, Linking, Operations, Bringing Unity, Lots of Adjustments, All Related                   | Collection of related changes                                  | `Globular Cluster(Refactor): Refactor multiple API endpoints for consistency`           |
|                                   | Void                       | Vanished, Obliterated, Irrelevant, Deleted                                                         | Removal of a module, component, or feature                     | `Void: Remove unused user settings module`                                              |
| **Astronomical Concepts**         | Gravity                    | Glitch Resolution, Adjusting Versions, Integrating, Troubleshooting Yielding                       | Resolving merge conflicts or dependencies                      | `Gravity: Resolve merge conflicts in feature/new-navigation branch`                     |
|                                   | Dark Matter                | Debugging And Resolving Mysterious Attributes, Tricky issues Removed                               | Fixing unknown or mysterious bugs                              | `Dark Matter: Fix intermittent crash on user login`                                     |
|                                   | Time Dilation              | Time Is Dilated, Improvements Leverage Agility, Time-Saving                                        | Improving code performance or reducing execution time.         | `Time Dilation(Backend): Optimize image processing algorithm for faster response`       |
|                                   | Spacetime                  | Scheduling, Planning, Adjusting Calendar Events, Coordinating Time                                 | Changes to date, time, or scheduling                           | `Spacetime(API): Fix timezone handling for event timestamps`                            |
|                                   | Gravitational Lensing      | Gravity Redirects Light, Altering Information Pathways                                             | Altering data or information flow                              | `Gravitational Lensing(Data): Refactor data pipeline for improved throughput`           |
|                                   | Cosmic String              | Connecting Our Sections, Merging Together, Interlinking New Groups                                 | Connecting code parts                                          | `Cosmic String(API): Connect user service with authentication middleware`               |
|                                   | Quantum Fluctuation        | Quick Unpredictable Adjustments, Noticed Tiny Unexpected Modification                              | Small, random change                                           | `Quantum Fluctuation: Fix typo in error message`                                        |
|                                   | Hawking Radiation          | Hastily And Willingly Killing Redundancies, Ageing Dead-ends, Tidying In Order, Obliterating Noise | Removing technical debt                                        | `Hawking Radiation: Remove unused CSS classes and refactor styles`                      |
|                                   | Quantum Entanglement       | Quantum Effects Never Tangled, Greater Efficiency, Linked Adjustments                              | Establishing close relationships between code parts            | `Quantum Entanglement(API): Tightly couple user profile and order history endpoints`    |
|                                   | Gravitational Redshift     | Gravity Reduces Efficiency, Degraded Speed, Shift Happens                                          | Slowing down or reducing code performance                      | `Gravitational Redshift(UI): Disable unnecessary animations for low-end devices`        |
| **Space Exploration**             | Space Probe                | Surveying, Planning, Analysing, Checking Every Nook                                                | Testing new features or technologies                           | `Space Probe(AI): Experiment with ChatGPT integration for customer support`             |
|                                   | Space Station              | Setting Up The Area, Testing In Orbit, Optimising New                                              | Creating or improving environments                             | `Space Station(DevOps): Set up new development environment with Docker`                 |
|                                   | Rocket Launch              | Releasing Our Code, Keenly Entering The Production                                                 | Deploying to production                                        | `Rocket Launch(v1.5): Deploy new version to production with enhanced security features` |
|                                   | Spacewalk                  | Swift Patches And Lookout Work, Keeping Systems Extra safe                                         | Urgent production hotfixes                                     | `Spacewalk(Database): Fix critical database connection issue causing downtime`          |
|                                   | Space Elevator             | Streamlined Access, Providing Easy Vertical On boarding, Lifting Entries                           | Making code base more accessible                               | `Space Elevator(API): Add new public API endpoints for third-party integrations`        |

## Commit Message Management with Cosmic Types

### What are they?

- **Commit Message Management:** The practice of writing clear, consistent, and informative commit messages to improve project collaboration and understanding.
- **Cosmic Commit Types:** A specific convention for commit messages that uses terms and concepts from astronomy and space exploration to categorise changes. This makes messages more engaging and easier to interpret.

### Why use commit message management?

- **Enhanced Collaboration:** Clear messages help team members understand the context and purpose of each change.

- **Improved History Tracking:** Well-structured commit logs make it easier to trace the development process, find specific changes, and generate meaningful changelogs.

- **Streamlined Review:** Concise and descriptive messages simplify code reviews and help identify potential issues faster.

- **1. Team Adoption:**

  - **Discuss and Agree:** Initiate a conversation with your team about using cosmic commit types. Explain the benefits, share this comprehensive guide, and gather feedback.
  - **Customise:** Collaboratively decide on the specific commit types you want to use. You can start with the comprehensive list provided here and tailor it to your project's specific needs and preferences.
  - **Document:** Create a clear and concise reference document outlining the chosen commit types, their meanings, and examples. Make this document easily accessible to all team members.

  **2. Implementation:**

  - **Manual Approach:** You can start using cosmic commit types manually by simply adhering to the `<type>(<scope>): <short summary>` format in your commit messages.
  - **Git Commit Template:** Create a Git commit template file (e.g., `.gitmessage`) to automatically populate the commit message format in your editor. This can help enforce consistency and remind contributors of the available commit types.
  - **Git Hooks:** Utilize Git hooks, like the `prepare-commit-msg` hook, to validate your commit messages and ensure they conform to the chosen format.
  - **Automated Tools:** Consider leveraging tools like `commitizen` or `cz-cli` that provide interactive prompts for creating commit messages according to your chosen convention. These tools can streamline the process and enforce consistency across your team.

  **3. Continuous Improvement:**

  - **Regular Review:** Periodically review your team's commit history to ensure consistent usage of the cosmic commit types and identify any areas where the format could be refined or improved.
  - **Feedback Loop:** Encourage open communication and feedback from your team members about the effectiveness of the chosen commit types and any suggestions for improvement.
  - **Iterative Refinement:** Don't be afraid to experiment and adapt the commit types to better suit your evolving project needs. The key is to find a system that works well for your team and enhances your Git workflow.

  **4. Continuous Improvement:**

  - **Encourage Creativity:** While maintaining consistency, allow team members to add their own flair and personality to the commit messages within the established framework.
  - **Celebrate Milestones:** Use special event commit types like "Moon Landing" to celebrate significant achievements and keep your team motivated.
  - **Integration with Other Tools:** Explore integration options with your issue tracking system, CI/CD pipeline, or documentation tools to automate processes and maximise the benefits of using cosmic commit types.

  By embracing this comprehensive guide and incorporating cosmic commit types into your Git workflow, you can transform your commit history into a vibrant, informative, and enjoyable reflection of your project's journey.

  The overall goal of Cosmic Commits is to make Git commit messages more informative, engaging, and enjoyable for developers, ultimately leading to better collaboration, maintainability, and understanding of the project's history.

### Why automate commit messages?

While Angular Commit Message Conventions provide a clear and structured format, enforcing it manually can be cumbersome and error-prone. Automated commit message generation tools help you:

- **Ensure Consistency:** All commit messages adhere to the convention, making the Git history more organized and easier to analyze.
- **Save Time:** Contributors don't have to manually format messages, leading to a more efficient workflow.
- **Reduce Errors:**  The tool guides contributors to create valid messages, preventing typos or inconsistencies.

### Why use cosmic commit types specifically?

- **Descriptive:** Terms like "Star" (new feature) or "Comet" (bug fix) are instantly recognisable and convey the nature of the change at a glance.
- **Engaging:** The cosmic theme adds a fun and memorable element to commit messages.
- **Standardised:** Provides a shared vocabulary and structured format for commit messages, improving consistency across the team.

### Why cosmos commit type ?

Cosmic commit types offer a unique and engaging way to categorise and describe changes in your Git commit history. Here's why they are beneficial:

**Enhanced Clarity and Communication:**

- **Descriptive Labels:** Using terms like "Star" for new features, "Comet" for bug fixes, or "Nebula" for refactoring instantly conveys the nature of the change to anyone reading the commit log. This improves communication and understanding within the team.
- **Visual Scanning:** The use of vivid imagery associated with celestial bodies and events makes it easier to quickly scan through a commit history and identify specific types of changes.
- **Contextual Information:** The optional addition of a scope within the commit message provides further context about which part of the code base was affected (e.g., "Star(UI)" for a new UI feature).

 **Improved Organisation and Maintainability:**

- **Structured Format:** The consistent format of cosmic commit messages (e.g., "Type(Scope): Short summary") makes the commit history more organised and easier to parse. This helps with tasks like generating change logs or filtering commits based on specific criteria.
- **Streamlined History:** A well-organised commit history makes it easier to track the evolution of the project, identify patterns, and quickly pinpoint the introduction of specific changes.

**Increased Engagement and Fun:**

- **Creative Expression:** The cosmic theme adds a touch of personality and fun to the often mundane task of writing commit messages. It can make the development process more enjoyable and engaging for the team.
- **Shared Vocabulary:** Using a common set of commit types fosters a sense of shared understanding and camaraderie within the team. It can also serve as a fun conversation starter or icebreaker.

**Automation and Tooling:**

- **Change log Generation:** Many tools can automatically generate change logs or release notes by parsing commit messages. Cosmic commit types make this process even easier by providing a clear structure and consistent vocabulary that tools can easily understand.
- **Issue Tracking Integration:** If you reference issue numbers in your commit messages, some tools can automatically link commits to their corresponding issues, streamlining your workflow and keeping your project management tools up-to-date.

### Commit message evolutions

```git
commit bb43c1df1b2ecd0a0d6a9c6615f51776af245862
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Mon Jul 22 19:44:50 2024 +0200

    Quasar(change log): update the change log
    
    update the change log to the latest commit
    
    The following changes were made:
    
            * prepare release
    
    The changes :
    
    Co-authored-by: Willy Micieli <otechdo@otechdo.com>

commit f164899688a0f359db45e02459c5c8cb8c32ee5f
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Mon Jul 22 19:41:13 2024 +0200

    Comet(main): rename function and variables
    
    fix bad function and variables name
    
    The following changes were made:
    
            * clean code
    
    The changes :
    
    Co-authored-by: Willy Micieli <otechdo@otechdo.com>

commit b469c66a148be1bd4fadc334480eeb4ebe9cefb1
Merge: 6966cf8 d38e183
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Mon Jul 22 19:13:28 2024 +0200

    Merge pull request #5 from otechdo/main
    
    Main

commit d38e183b5bacce2da3ade7fecc01340b8f3c7e01
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Mon Jul 22 19:10:59 2024 +0200

    Big Bang(feat): add new features
    
    add cargo command
    
    The following changes were made:
    
            * clean code
    
    The changes :
    
    Co-authored-by: Willy Micieli <otechdo@otechdo.com>

commit 1c7204878c94371c646e5cfa0ba44a9f87b04b72
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Mon Jul 22 17:51:10 2024 +0200

    Big Bang(main ): create the main branch
    
    prepare the deletions of the master branch
    
    The following changes were made:
    
            * rename the release branches
    
    The changes :
    
    Co-authored-by: Willy Micieli <otechdo@otechdo.com>

commit 6966cf883c5b9fc27f5d17a9832a4ec1ee1a6e03
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Mon Jul 22 17:22:30 2024 +0200

    Big Bang(change log): generate change log
    
    start 4.0.0 version
    
    The following changes were made:
    
            * prepare change log for the next version
    
    The changes :
    
    Co-authored-by: Willy Micieli <otechdo@otechdo.com>

commit d789489f40db46aac48f1b7219c76c9e4394347a
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Mon Jul 22 17:20:29 2024 +0200

    Comet(command): remove command bad function
    
    fix bug remove support of mercurial
    
    The following changes were made:
    
            * fix bugs
    
    The changes :
    
    Co-authored-by: Willy Micieli <otechdo@otechdo.com>

commit 56fa02c8ba0aa36fc7fe3234a52ad8d710a2ce6c
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Mon Jul 22 16:30:19 2024 +0200

    Comet(use flow method): replace committer by flow
    
    remove code
    
    The following changes were made:
    
            * clean code
    
    The changes :
    
    Co-authored-by: Willy Micieli <otechdo@otechdo.com>

commit 81eba39e6766139900255d8f14b415688a5c5224
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Mon Jul 22 16:19:42 2024 +0200

    Comet(main): add missing output text on bad text
    
    fix missing output if text is bad
    
    refactor the version to 4.0.0
    
    The following changes were made:
    
            * next version is better
    
    The changes :
    
    Co-authored-by: Willy Micieli <otechdo@otechdo.com>

commit abf5ed35ab2ca6bf77218db0ef053306a6a686cd
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Mon Jul 22 16:00:57 2024 +0200

    Nebula(committer): add missing verification
    
    check if the code is valid first
    
    check if the repository is initialized
    
    The following changes were made:
    
            * fix bug on git or mercurial command tentative
    
    The changes :
    
    Co-authored-by: Willy Micieli <otechdo@otechdo.com>

commit 96008107e1c2105d628e6c026645fd02bd16a19f
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Mon Jul 22 15:41:27 2024 +0200

    Big Bang(commit): check commit typo
    
    if text is valid jump to the next steps
    
    or continue to write the bad commit part
    
    The following changes were made:
    
            * limit the bad English commit message
            * check the text before save the commit
    
    The changes :
    
    Co-authored-by: Willy Micieli <otechdo@otechdo.com>

commit 24568f01cb909b9ab913a6b39281ab428bc23208
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Mon Jul 22 02:18:22 2024 +0200

    Comet(ignore): update gitignore
    
    add .vscode in gitignore
    
    The following changes were made:
    
            * fix missing gitignore value
    
    The changes :
    
    Co-authored-by: Willy Micieli <otechdo@otechdo.com>

commit 237657e2b10a56d24725233f11d9a0d0d6159e96
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Mon Jul 22 02:10:51 2024 +0200

    Comet(send): remove pull command on send modifications
    
    fix push bugs
    
    The following changes were made:
    
            * fixes bug on git pull with no remote
    
    The changes :
    
    Co-authored-by: Willy Micieli <otechdo@otechdo.com>

commit 9aaea5badafb4efc7f67cb1e9e3af9bbc901b0f6
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Mon Jul 22 02:06:59 2024 +0200

    Black Hole(flow): remove flow package
    
    clean repository use only commiter
    
    The following changes were made:
    
            * remove bin flow
            * fix flow commit bug
            * fix flow generate change log bug
    
    The changes :
    BREAKING CHANGE: removed bin flow to intgrate feature in comiter
    
    Co-authored-by: Willy Micieli <otechdo@otechdo.com>

commit f58f664e0639b731ed5a0f2f50dfb50fe511f520
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Sun Jul 21 11:41:24 2024 +0200

    Comet(license): fix missing licenses
    
    add missing licenses
    
    fix bug on generate change log
    
    The following changes were made:
    
            * add linces in all projets
    
    The changes :
    
    Co-authored-by: Willy Micieli <otechdo@otechdo.com>

commit b097cb67d512f3c0dec915603cf34229c8adfc93
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Sun Jul 21 11:32:22 2024 +0200

    Nebula(repository): use workspace
    
    start flow executable
    
    update project structure
    
    The following changes were made:
    
            * separate project in two smal project
            * starting flow project
    
    The changes :
    
    Co-authored-by: Willy Micieli <otechdo@otechdo.com>

commit f13bcd2fd84f17e81d4eda9f89aa4c529cdf7e28
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Fri Jul 19 17:53:21 2024 +0200

    Comet(send): pull modifications before send
    
    download teams modification pefore send modifications
    
    The following changes were made:
    
            * resolve push rejeted if modifs not downloaded
    
    The changes :
    
    Co-authored-by: Willy Micieli <otechdo@otechdo.com>

commit f1aa2ca7f098bd0cfc96bd225d2d506f78ef2b61
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Fri Jul 19 09:03:39 2024 +0200

    Rocket Launch(3.0.0): launch new version
    
    bug fixes and commit improvements
    
    The following changes were made:
    
            * add confirmation
            * fix bugs
            * reformat log output
            * add git author name
            * add git author email
    
    The changes :
    
    Co-authored-by: Willy Micieli <otechdo@otechdo.com>

commit 53abddb16cde290c255d9f56fa8a9d1c87497995
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Fri Jul 19 08:52:03 2024 +0200

    Quasar(changelog): update changelog
    
    add missing change log commits
    
    The following changes were made:
    
            * update changelog
    
    The changes :
    
    Co-authored-by: Willy Micieli <otechdo@otechdo.com>

commit 6764b9542ffa3837971e06d83038ebeb7e1d596c
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Fri Jul 19 08:43:57 2024 +0200

    Comet(commit): fix issue n°4
    
    remove space between username and email
    
    Email and username on se same line
    
    The following changes were made:
    
            * fix bad format on footer authored by
    
    The changes :
    
            Fixes #4
    
    Co-authored-by: Willy Micieli <otechdo@otechdo.com>

commit c7457c0a12aeb11d45d5e042cb0c56b34010e889
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Fri Jul 19 08:41:00 2024 +0200

    Comet(commit): fix commit output format
    
    clena log commit format
    
    fix identation bug
    
    The following changes were made:
    
            * clean commit output
            * fix commit issues not closed
            * better output visibility
    
    The changes :
    
            Fixes #1
    
            Fixes #2
    
            Fixes #3
    
    Co-authored-by: Willy Micieli
     <otechdo@otechdo.com>

commit 2fcbff045cab6176b180d2999095677ebd883d1f
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Fri Jul 19 08:24:23 2024 +0200

    Big Bang: Initial commit or major feature start(commit generation): improve commit generation
    
    add confirmation before validate
    
    new commit description can be have lines
    
    The following changes were made:
    
            * add confirm commit information
            * clean commit output
            * add commit author name
            * add commit author email
            * separate commit in multiples functions
    
    The changes were made:
            Fixes 1#1
    Fixes 2#2
    Fixes 3#3
    
    Co-authored-by: Willy Micieli
     <otechdo@otechdo.com
    >

commit 794d47d9075b12a435fc39f84aae6e572eb4da37
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Fri Jul 19 07:14:40 2024 +0200

    Big Bang(changelog): create new changelog
    
    add next release changelog
    
    The following changes were made:
    
            * add changelog for release 2.3.0
    
    fix missing change log

commit a8415c4967755e48423eba701dc7db01e003ef3b
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Fri Jul 19 07:12:58 2024 +0200

    Black Hole(readme): remove undeveloped feature
    
    remove explication in  readme not currently developed
    
    The following changes were made:
    
            * fix readme doc
            * upgrade release
    
    fix commit message format
    add example

commit f34e0a6023073afac4c5fc8cf6a8609a2b49aba8
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Fri Jul 19 06:57:49 2024 +0200

    Quasar(changeslog): update changelog
    
    add latest commit in changelog
    
    The following changes were made:
    
    * add missing commit
    
    clean changelog

commit ee30651ce11bc6720ccb201d6e3dc5f0655cc6ec
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Fri Jul 19 06:54:08 2024 +0200

    Black Hole(commit types): remove basic types constant
    
    use the commit type with help message
    
    The following changes were made:
    
    * remove a choice
    * simplify commit types selection
    * remove TYPES contants
    
    improve commit input rapidity

commit 65747856bad9e526a7912887be715a5e17e1f4f4
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Fri Jul 19 06:45:20 2024 +0200

    Dwarf Planet(change log): add license content in change log
    
    add missing lince content after rank
    
    The following changes were made:
    
    * add missing lincense content
    
    display license in changelog

commit 42198b6a0a02ded3686bc3a721769378e9a75967
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Fri Jul 19 06:31:55 2024 +0200

    Quasar(changeslog): add last commit
    
    add mising new changes
    
    The following changes were made:
    
    * regenerate changelog
    
    refresh changelog content

commit 79b594934f580f66a4ac28b186ced13ab17190f9
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Fri Jul 19 06:29:57 2024 +0200

    Comet(commiter): fix docs bad typo
    
    clean main help text documentation
    
    The following changes were made:
    
    * replace text with correct values
    
    clean help documentation

commit 1d58a188c9bc97ab3255efbae9cbacbbe5381b6f
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Fri Jul 19 06:24:21 2024 +0200

    Big Bang(rank): add rank in changelog
    
    add team stats in changeslog
    
    The following changes were made:
    
    * in order to see who works
    
    add commits stats

commit 6a2e729bc8361129f697a008f025feb8e0e030eb
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Fri Jul 19 06:13:50 2024 +0200

    Quasar(changes): use ordened list for changelog
    
    replace title by a ordoned list
    
    The following changes were made:
    
    * improve lisibilty
    
    clean change log

commit 39743cb4a68767001e2635f9322b38b59cb311b0
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Fri Jul 19 06:00:21 2024 +0200

    Comet(changelog): fix empty summary in changelog
    
    fix bug with empty changelog commit message
    
    The following changes were made:
    
    * add missing log message for new release
    
    fix bug on changelog generation

commit 8a33fd3c7f862a9835992d375259733e02137f5a
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 21:39:49 2024 +0200

    Comet(commit footer): fix footer indentation output
    
    remove space between left and text begining
    
    The following changes were made:
    
    * clean output of the footer
    * update change log
    
    clean footer

commit aed2d18ebba0ce63af69e662a762559909de44d1
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 21:34:41 2024 +0200

    Comet(commit): separate summary and description
    
    add missing new line between sumary and description
    
    The following changes were made:
    
    * update output of git log command
    
     update changes log

commit 8df5224a51dfb6c4176677425b9d27c891d99099
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 21:07:12 2024 +0200

    Comet(commit message): fix commit message format
    use corectness commit format
    
    The following changes were made:
    
    * add a empty line between body and why
    * fix missing footer text
    
    * fix missing data

commit bf8957c13df1c30d4090aa86753f39011d75ddb9
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 20:59:03 2024 +0200

    Big Bang(commit message): change commit message format
    add more detailed commit change
    The following changes were made:
    
    * check the length of the summary
    * add why changes

commit 3742772efa7ce2a2ee6446c60b5363583d39d917
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 20:23:00 2024 +0200

    Nebula(readme): rewrite readme

commit 1087c708c42177145f1e73c2ff768c78856b92e8
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 19:05:00 2024 +0200

    Quasar(readme): update reamde documentation

commit 37aaee8e63065acc5e4e5d928e07924662d75ada
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 19:01:25 2024 +0200

    Big Bang(rank): add commiter rank

commit d2cf0c07cb26d531d88fdbc6ef8d601342532c4b
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 17:38:26 2024 +0200

    Quasar(changes): update change log

commit 336b0539376025b33748cf8be1d0411ab1d0bf04
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 17:36:44 2024 +0200

    Comet(commit types): fix duplicate first contact type

commit 0e40d5ffc0a40590559e3de7b4a9ab820ebc2960
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 17:33:31 2024 +0200

    Lunar Transit(readme): add changelog link for release 222

commit 81df5a0ca9ac95ea0db468e87e47671365ddba10
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 17:26:49 2024 +0200

    Quasar(changes): update changes log

commit f1ec037d8897d539d65e2a9f9cd3f65e855f48e6
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 17:25:18 2024 +0200

    Neutron Star(commit): run commit after generate change log

commit cde6cd9be1104fb98a312055482d226642c79ba2
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 17:22:05 2024 +0200

    Black Hole(change log): remove unseless callback

commit c95ebd18ea745ae60004740b04cc49e88c5a6460
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 17:17:49 2024 +0200

    Neutron Star(change log): auto update change log after all commits

commit fdf9d0c8377f6688354d968e8e40be4f21efeaf8
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 17:13:46 2024 +0200

    Quasar(readme): add generation change log comand

commit e740f024459ac4adcadefc6b402ffc8a85b298e7
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 16:57:53 2024 +0200

    Quasar(changelog): update change log

commit b5dae8291735c4aa2fcda704158d521c6f80cd46
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 16:56:08 2024 +0200

    Big Bang(generate log): can generate change log

commit c49191db78796d21bcbd5b15b34f4d4050a6ee69
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 13:24:38 2024 +0200

    Nebula(add): change add command question

commit b6b605b9de84729b0ea09ed52cd2655e359af61f
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 13:08:58 2024 +0200

    Big Bang(patch): start patch creation
    create automatic pach by the last commit

commit b92758d0a6e4251583c0bb30480a2bc63dd98ed0
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 11:10:08 2024 +0200

    Launch(2.1.1): auto publish new version for rust

commit 9550bc6d59f02001d478d53f78001ec62e504ab7
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 11:02:20 2024 +0200

    Nebula(send): remove name of default remote

commit 1b3f4d8a4adbf386aaa1022c2ab75ba850087c11
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 10:54:38 2024 +0200

    Big Bang(tag): start tag creation possibility

commit e1063df1b785c41a30ff5f1d1ef0a6b758ff7efe
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Thu Jul 18 07:37:39 2024 +0200

    Star(2.1.0): start new version

commit 2ca424403b0463ce5b6b9a8b5d7101638c198420
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Wed Jun 26 07:31:45 2024 +0200

    Rocket Launch(2.0.0): run git init automatic
    initialize git on new directory or no git project
    fix no git project

commit db6b5719c1a4740471d197be407b4e4f8fc90493
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Tue Jun 25 16:29:28 2024 +0200

    Rocket Launch(1.3.1): refactor readme

commit feb484cc39936cda227642feaaa8b2b9dfde4dbc
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Tue Jun 25 16:26:50 2024 +0200

    Quasar(readme): cleanup readme documentation
    arrange readme add important info
    add mnemonics and Table of contents

commit 4df64e94a8694d6a5dfc21654c6c465cd0ea63e1
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Tue Jun 25 13:34:26 2024 +0200

    Launch(exe): first stable version
    clean all possible bug
    can be use

commit dd6194ce4eab384f6080c9e9d7f292f17afd116e
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Tue Jun 25 13:26:31 2024 +0200

    Spacewalk(commit): use cargo fmt on cargo project only
    fix using cargo fmt on  no cargo project
    can be use now on different project

commit 0fcc480e23a9bea229a947efe9f028261a893fe3
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Tue Jun 25 13:22:25 2024 +0200

    Supernova(prog): open the program to no cargo project
    Offer the possibility to use commiter for any project
    work on all project using git

commit 58773a2aaef4e61a236fe468ce8ab6ad21761bf4
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Tue Jun 25 13:11:53 2024 +0200

    Comet(commit): remove duplicate line on commit message
    
    fix no display body and footer
    
    refactor commit message format

commit a438c9f05f3e4bcf30aabc49873d0abef80b291c
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Tue Jun 25 13:08:43 2024 +0200

    Supernova(commit): add missing commit structure
    
    add commit body and stucture

commit c90bd10464e1ef6c83a4ed2cd9cd50f57ea75587
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Tue Jun 25 13:00:51 2024 +0200

    Dwarf Planet(main): add missing help

commit d504d6f4c7ba78fa2b68589773a0a68c19dfa858
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Tue Jun 25 12:55:39 2024 +0200

    Supernova(main): clean type

commit 16202b8f3fb04555b88ba94b05810e7ce144f939
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Tue Jun 25 08:13:24 2024 +0200

    Black Hole(readme): remove unseless title

commit e7aa910c9aa9c67edf6193c1efb21c07f98b2c00
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Mon Jun 24 22:02:38 2024 +0200

    Quasar(docs): update readme

commit dc00113ec29e8be64c772e990268bef951ec2659
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Mon Jun 24 20:25:20 2024 +0200

    Quasar(Docs): update readme

commit 83321bbe603b38099bb609446418e37e1a7e39cf
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Mon Jun 24 19:46:45 2024 +0200

    Pulsar(readme): add documentation

commit 4175c4ce40122fc8c90f9cb3372ca73c4a92358f
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Mon Jun 24 18:34:20 2024 +0200

    Black Hole(docs): remove unseless commit types doc

commit 1a79979e766374e0268349cea9a2c8499ece9535
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Mon Jun 24 18:29:28 2024 +0200

    Parallax(main): remove unnessary parenthehis

commit 60ac49b1c3ce93495c4c223a05726f9694195970
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Mon Jun 24 18:26:40 2024 +0200

    Singularity)(scope): simplify scope

commit 94432fc2dd50b9ba3856e6ebed8814a512405fdb
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Mon Jun 24 18:21:07 2024 +0200

    Black Hole: change commit types

commit 9a405b6a1d9b535bdbacda478263b6429d5fd702
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Mon Jun 24 14:19:57 2024 +0200

    feat(cli): add publish function

commit 39dd7bb08426a74395d6f74b13aad80dc027ab9b
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Mon Jun 24 14:16:46 2024 +0200

    build(cli): update licence

commit b299cc3e38a54ecca1d5249dd8bcb409e9bf8f32
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Mon Jun 24 14:13:05 2024 +0200

    docs(cli): update readme

commit e24685035b72200af6e5bdd6d437183ef78c74bf
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Mon Jun 24 10:45:47 2024 +0200

    refactor(view): add uppercase to bye

commit 7d48789dca3b56317d44698ea786450c1c6aaff6
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Mon Jun 24 10:43:24 2024 +0200

    feat(cli): add scope

commit 0f0c4daade202e6d0caa062d15e8621075fa300b
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Mon Jun 24 09:22:40 2024 +0200

    fix: remove duplicate type

commit ee136a2e8b14f1f83fca59e1fef13f08d6fcbb4d
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Mon Jun 24 09:20:21 2024 +0200

    feat: add commit type

commit 9d3d1eed725e24bd1ca834fa8e93c608ae1a0870
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Mon Jun 24 04:00:56 2024 +0200

    feat: add commit types

commit 0681a56aeaa3b1b9687838e58d77a0277c6aa979
Author: Willy Micieli <micieli@vivaldi.net>
Date:   Mon Jun 24 03:51:54 2024 +0200

    init: start project
```
