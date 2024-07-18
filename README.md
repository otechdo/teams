- Commiter
  - [Why use it ?](https://file+.vscode-resource.vscode-cdn.net/var/home/otechdo/Ness/Rust/otechdo/commiter/README.md#why-use-it-)
  - [Why cosmos commit type ?](https://file+.vscode-resource.vscode-cdn.net/var/home/otechdo/Ness/Rust/otechdo/commiter/README.md#why-cosmos-commit-type-)
- Installation
  - [Archlinux user install](https://file+.vscode-resource.vscode-cdn.net/var/home/otechdo/Ness/Rust/otechdo/commiter/README.md#archlinux-user-install)
- Usage
  - [Commit Message Format](https://file+.vscode-resource.vscode-cdn.net/var/home/otechdo/Ness/Rust/otechdo/commiter/README.md#commit-message-format)
- [Commit Types](https://file+.vscode-resource.vscode-cdn.net/var/home/otechdo/Ness/Rust/otechdo/commiter/README.md#commit-types)

# Committer

The "Cosmic Commits" project is a convention for writing Git commit messages that uses terms and concepts from astronomy and space exploration to categorise and describe changes made to a code base. This approach aims to make commit messages more informative, engaging, and easier to understand for developers and project stakeholders.

Key aspects of the Cosmic Commits project:

- **Categorisation:** Commits are categorised based on their nature, such as:

  - **Core Changes:** New features, bug fixes, refactoring, performance improvements, and documentation updates.
  - **Maintenance & Infrastructure:** Code cleanup, testing, dependency updates, and infrastructure changes.
  - **Special Events:** Major milestones, releases, or significant code removals.
  - **Communication & Collaboration:** Project management updates, urgent fixes, and improvements to communication and documentation.
  - **Celestial Events, Celestial Objects, Astronomical Concepts, and Space Exploration:** Additional categories with more specific commit types inspired by the cosmos.

- **Descriptive Labels:** Each commit type is assigned a descriptive label that reflects its purpose, making it easy to identify the type of change at a glance. For example, "Star" for new features, "Comet" for bug fixes, and "Nebula" for refactoring.

- **Mnemonic Devices:** Some commit types come with mnemonic devices to help remember their meaning and usage. For example, "Comet" stands for "Code or Module Error Terminated."

- **Structured Format:** The project recommends a consistent commit message format, such as:

  ```
  <type>(<scope>): <short summary>
  
  [Optional body with more details]
  
  [Optional footer(s)]
  ```

- **Flexibility and Customisation:** The list of commit types is flexible and can be adapted to the specific needs and terminology of different projects. Teams are encouraged to create their own cosmic terms or modify existing ones to better suit their workflow.

- **Tooling:** The project can be enhanced with tools like `commitizen` or `cz-cli` to automate commit message creation and validation, ensuring consistency and adherence to the chosen conventions.

The overall goal of Cosmic Commits is to make Git commit messages more informative, engaging, and enjoyable for developers, ultimately leading to better collaboration, maintainability, and understanding of the project's history.

## Why use it ?

**1. Team Adoption:**

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
- **Integration with Other Tools:** Explore integration options with your issue tracking system, CI/CD pipeline, or documentation tools to automate processes and maximize the benefits of using cosmic commit types.

By embracing this comprehensive guide and incorporating cosmic commit types into your Git workflow, you can transform your commit history into a vibrant, informative, and enjoyable reflection of your project's journey.

**5. Commit Graph Example:**

```
* Big Bang: Initial project creation
    |\
    | *--* Star (UI): Implement initial user interface
    | *--* Star (API): Create basic API endpoints
    | *--* Nebula (Backend): Refactor project structure for modularity
    |    |\
    |    | *--* Comet (UI): Fix responsive design issue on mobile
    |    | *--* Pulsar (API): Optimize database queries for faster response
    |    *--* Launch (v0.1): First version deployed to staging
    |\
    | *--* Star (Search): Add new search functionality
    | *--* Solar Flare (Tests): Add unit and integration tests for search
    | *--* Quasar (Docs): Update documentation with search instructions
    |    |\
    |    | *--* Comet (Search): Fix bug in search result ranking
    |    *--* Launch (v0.2): Release new version with search feature to production
    |\
    | *--* Black Hole: Remove deprecated user management module
    | *--* Wormhole: Merge feature/payment-integration branch
    |    |\
    |    | *--* Star (Payments): Integrate Stripe payment gateway
    |    | *--* Solar Flare (Payments): Add tests for payment processing
    |    *--* Launch (v0.3): Release version with payment functionality
    |\
    | *--* Asteroid Belt: Cleanup unused code and optimize images
    | *--* Mission Control: Update project roadmap and assign tasks
    | *--* Spacewalk (Security): Patch critical vulnerability in authentication
    | *--* Moon Landing: Successfully launch version 1.0 to production
    *--* (ongoing development)
```

## Why cosmos commit type ?

Cosmic commit types offer a unique and engaging way to categorize and describe changes in your Git commit history. Here's why they are beneficial:

1. **Enhanced Clarity and Communication:**

- **Descriptive Labels:** Using terms like "Star" for new features, "Comet" for bug fixes, or "Nebula" for refactoring instantly conveys the nature of the change to anyone reading the commit log. This improves communication and understanding within the team.
- **Visual Scanning:** The use of vivid imagery associated with celestial bodies and events makes it easier to quickly scan through a commit history and identify specific types of changes.
- **Contextual Information:** The optional addition of a scope within the commit message provides further context about which part of the codebase was affected (e.g., "Star(UI)" for a new UI feature).

1. **Improved Organization and Maintainability:**

- **Structured Format:** The consistent format of cosmic commit messages (e.g., "Type(Scope): Short summary") makes the commit history more organized and easier to parse. This helps with tasks like generating changelogs or filtering commits based on specific criteria.
- **Streamlined History:** A well-organized commit history makes it easier to track the evolution of the project, identify patterns, and quickly pinpoint the introduction of specific changes.

1. **Increased Engagement and Fun:**

- **Creative Expression:** The cosmic theme adds a touch of personality and fun to the often mundane task of writing commit messages. It can make the development process more enjoyable and engaging for the team.
- **Shared Vocabulary:** Using a common set of commit types fosters a sense of shared understanding and camaraderie within the team. It can also serve as a fun conversation starter or icebreaker.

1. **Automation and Tooling:**

- **Change log Generation:** Many tools can automatically generate change logs or release notes by parsing commit messages. Cosmic commit types make this process even easier by providing a clear structure and consistent vocabulary that tools can easily understand.
- **Issue Tracking Integration:** If you reference issue numbers in your commit messages, some tools can automatically link commits to their corresponding issues, streamlining your workflow and keeping your project management tools up-to-date.

In summary, cosmic commit types offer a more descriptive, organized, and engaging way to document your project's history. They improve communication, streamline workflows, and add a touch of fun to the development process.

Would you like me to elaborate on any of these points or provide more examples of how cosmic commit types can be used in practice?

# Installation

```bash
cargo install zuu commiter cargo-watch
```

## Arch Linux user install

```bash
paru -S commiter
```

# Usage

```bash
cd repository && cargo-watch commiter
```

## Commit Message Format

```
<type>(<scope>): <short summary>

[Optional body with more details]

[Optional footer(s)]
```

# Commit Types

| Category                          | Commit Type                | Mnemonic                                                     | Description                                               | Example                                                      |
| :-------------------------------- | :------------------------- | :----------------------------------------------------------- | :-------------------------------------------------------- | :----------------------------------------------------------- |
| **Core Changes**                  | Star                       | Shiny Technology Added or Refined                            | New feature or enhancement                                | `Star(Auth): Implement two-factor authentication`            |
|                                   | Comet                      | Code or Module Error Terminated                              | Bug fix or error resolution                               | `Comet(UI): Fix responsive layout issue on mobile devices`   |
|                                   | Nebula                     | New Efficient Better Understandable Logic Achieved           | Code refactoring                                          | `Nebula(Backend): Refactor user management module for improved maintainability` |
|                                   | Pulsar                     | Powerful Upgrade, Less Sluggish, Agile Response              | Performance improvement                                   | `Pulsar(Database): Optimize queries for faster response times` |
|                                   | Quasar                     | Quick Adjustments for Superior Accuracy and Readability      | Documentation or clarity improvement                      | `Quasar(API): Update documentation with new endpoint parameters` |
| **Maintenance & Infrastructure**  | Asteroid Belt              | Adjustments, Sweeps, Tidy-ups, Elimination, Reordering of Items, Decrease Bloat | Code cleanup and maintenance                              | `Asteroid Belt: Remove unused CSS and optimize images`       |
|                                   | Solar Flare                | Securing Our Logic Against Regressions, Failures, and Latencies Actively, Rigorously Ensured | Adding or updating tests (unit, integration, end-to-end). | `Solar Flare(Payments): Add unit tests for payment processing module` |
|                                   | Dwarf Planet               | Details Warranted Attention, Refined Further, Polished Little Aspects Neatly Enhanced Tiny | Minor but essential updates or fixes.                     | `Dwarf Planet: Update project dependencies to latest versions` |
|                                   | Terraform                  | Technology Engineering Resources Readily Automated, Foundation of Reliable Management | Infrastructure changes                                    | `Terraform(AWS): Provision new EC2 instance for staging environment` |
| **Project Events**                | Black Hole                 | Big Legacy Aspects Consumed, Killing Heavy, Old Loads Entirely | Removing large chunks of code or features                 | `Black Hole: Remove deprecated user profile module`          |
|                                   | Wormhole                   | Weaving or Reconnecting Modules, Hitching onto Linked Elements | Merging branches or connecting code parts                 | `Wormhole: Merge feature/new-dashboard into develop branch`  |
|                                   | Big Bang                   | Birth of Initial Greatness, Beginning All New Growth         | Initial commit of a project or major feature              | `Big Bang: Initial project setup and scaffolding`            |
|                                   | Launch                     | Lifting Application Upward, New Code Entering Production     | Deploying to production or releasing a version            | `Launch(v1.2): Release new version with user profile customization` |
| **Communication & Collaboration** | Lightspeed                 | Lightening Speed Enhancements                                | Significant performance improvements                      | `Lightspeed(Frontend): Implement lazy loading for images`    |
|                                   | Mission Control            | Managing Changes, Issues, Scope, Teamwork, and Release On Time | Project management changes                                | `Mission Control: Update project roadmap and assign tasks for Q3` |
|                                   | Spacewalk                  | Swift Work Above Limits, Keeping All Systems Extra Safe      | Urgent hotfixes or critical production updates.           | `Spacewalk(Security): Patch critical vulnerability in authentication module` |
|                                   | Moon Landing               | Major Leaps Over Night, New Doors and Incredible Achievements | Completing major milestones or goals                      | `Moon Landing: Successfully launch beta version to select users` |
|                                   | First Contact              | Forge Initial Connections, Open New Territories              | Establishing initial connections or integrations          | `First Contact(API): Integrate with new payment provider's API` |
|                                   | Interstellar Communication | Informing, Sharing, Teaching, Educating, & Learning Lucidly & Clearly | Improving documentation or communication                  | `Interstellar Communication: Update wiki with troubleshooting guide for common errors` |
| **Celestial Events**              | Solar Eclipse              | Sun Escapes, Legacy Code Lurks                               | Temporarily masking functionality.                        | `Solar Eclipse(Feature): Temporarily disable new onboarding flow for testing` |
|                                   | Supernova                  | Sudden Unbelievable Performance Revolution, New Version Arrives | Major, transformative change or improvement.              | `Supernova(Architecture): Migrate to microservices architecture` |
|                                   | Meteor Shower              | Many Edits, Tiny Overall Result, Overhaul Routines           | Series of small changes or fixes.                         | `Meteor Shower(UI): Fix various UI inconsistencies and typos` |
|                                   | Solar Wind                 | Swift Optimization, Light and Refactored, Agile Wind         | Refactoring code structure.                               | `Solar Wind(Backend): Refactor authentication middleware for improved readability` |
|                                   | Lunar Eclipse              | Less Used, Unnecessary Now, Available Returns                | Temporarily disabling a feature.                          | `Lunar Eclipse(Payments): Disable PayPal integration due to maintenance` |
|                                   | Cosmic Dawn                | Creating Original, Simple, Minimal Initial Draft             | Initial implementation of a feature.                      | `Cosmic Dawn(Search): Initial implementation of basic search functionality` |
|                                   | Solar Storm                | Sudden Transformations Occur Rapidly, Modifications          | Rapid, impactful changes.                                 | `Solar Storm(Refactor): Overhaul data processing pipeline for improved performance` |
|                                   | Lunar Transit              | Little Update, Now Adjustments Require Testing               | Minor, temporary change.                                  | `Lunar Transit(Config): Temporarily adjust logging level for debugging` |
|                                   | Perihelion                 | Perfect Ending, Refined, Improved, High Efficiency, Low Obstacles, Near Goal | Significant milestone or feature completion.              | `Perihelion: Successfully complete user acceptance testing for new dashboard` |
|                                   | Aphelion                   | Away From Perfection, High Effort, Long Overhaul, Intense Overhaul, Obstacles | Refactor, dependency update, or architecture change.      | `Aphelion: Upgrade to React 18 and refactor components`      |
| **Celestial Objects**             | White Dwarf                | Writing, Improving, Detailed Documentation For All           | Improving code comments or documentation                  | `White Dwarf(API): Add detailed documentation for new endpoints` |
|                                   | Red Giant                  | Refactoring, Enhancing, Growing, Increasing, Adding New Things | Expanding a feature or functionality                      | `Red Giant(Payments): Add support for Apple Pay and Google Pay` |
|                                   | Neutron Star               | New Efficient Utility, Tweaks, Robust Optimization, Nimble Solution | Optimizing code for performance                           | `Neutron Star(Search): Optimize search algorithm for faster results` |
|                                   | Binary Star                | Bringing In New And Revised, Yielding Integrated Results     | Merging features or components                            | `Binary Star: Merge user authentication and authorization modules` |
|                                   | Brown Dwarf                | Barely Developed, Requires Work, Ongoing Development For Future | Undeveloped feature with potential                        | `Brown Dwarf(Social): Initial prototype for social sharing feature` |
|                                   | Quark Star                 | Questionable, Unstable, Anticipated Results, Risky, Keen Experiment | Experimental or speculative change                        | `Quark Star(AI): Experiment with integrating GPT-3 for content generation` |
|                                   | Rogue Planet               | Refactoring Or Generating Operations, Unique Path, Leaping Ahead | Independent change unrelated to the main codebase         | `Rogue Planet: Create standalone script for data migration`  |
|                                   | Stellar Nursery            | Starting To Enhance, Laying Layers, Launching New Requirements | Creating new components                                   | `Stellar Nursery(UI): Add new component library for design system` |
|                                   | Planetary Nebula           | Pruning, Leaving, Abandoning, Nostalgic Era, Totally Removed | Removal or deprecation of a component                     | `Planetary Nebula: Remove legacy image carousel component`   |
|                                   | Globular Cluster           | Gathering, Linking, Operations, Bringing Unity, Lots of Adjustments, All Related | Collection of related changes                             | `Globular Cluster(Refactor): Refactor multiple API endpoints for consistency` |
|                                   | Void                       | Vanished, Obliterated, Irrelevant, Deleted                   | Removal of a module, component, or feature                | `Void: Remove unused user settings module`                   |
| **Astronomical Concepts**         | Gravity                    | Glitch Resolution, Adjusting Versions, Integrating, Troubleshooting Yielding | Resolving merge conflicts or dependencies                 | `Gravity: Resolve merge conflicts in feature/new-navigation branch` |
|                                   | Dark Matter                | Debugging And Resolving Mysterious Attributes, Tricky issues Removed | Fixing unknown or mysterious bugs                         | `Dark Matter: Fix intermittent crash on user login`          |
|                                   | Time Dilation              | Time Is Dilated, Improvements Leverage Agility, Time-Saving  | Improving code performance or reducing execution time.    | `Time Dilation(Backend): Optimize image processing algorithm for faster response` |
|                                   | Spacetime                  | Scheduling, Planning, Adjusting Calendar Events, Coordinating Time | Changes to date, time, or scheduling                      | `Spacetime(API): Fix timezone handling for event timestamps` |
|                                   | Gravitational Lensing      | Gravity Redirects Light, Altering Information Pathways       | Altering data or information flow                         | `Gravitational Lensing(Data): Refactor data pipeline for improved throughput` |
|                                   | Cosmic String              | Connecting Our Sections, Merging Together, Interlinking New Groups | Connecting code parts                                     | `Cosmic String(API): Connect user service with authentication middleware` |
|                                   | Quantum Fluctuation        | Quick Unpredictable Adjustments, Noticed Tiny Unexpected Modification | Small, random change                                      | `Quantum Fluctuation: Fix typo in error message`             |
|                                   | Hawking Radiation          | Hastily And Willingly Killing Redundancies, Ageing Dead-ends, Tidying In Order, Obliterating Noise | Removing technical debt                                   | `Hawking Radiation: Remove unused CSS classes and refactor styles` |
|                                   | Quantum Entanglement       | Quantum Effects Never Tangled, Greater Efficiency, Linked Adjustments | Establishing close relationships between code parts       | `Quantum Entanglement(API): Tightly couple user profile and order history endpoints` |
|                                   | Gravitational Redshift     | Gravity Reduces Efficiency, Degraded Speed, Shift Happens    | Slowing down or reducing code performance                 | `Gravitational Redshift(UI): Disable unnecessary animations for low-end devices` |
| **Space Exploration**             | Space Probe                | Surveying, Planning, Analyzing, Checking Every Nook          | Testing new features or technologies                      | `Space Probe(AI): Experiment with ChatGPT integration for customer support` |
|                                   | Space Station              | Setting Up The Area, Testing In Orbit, Optimizing New        | Creating or improving environments                        | `Space Station(DevOps): Set up new development environment with Docker` |
|                                   | Rocket Launch              | Releasing Our Code, Keenly Entering The Production           | Deploying to production                                   | `Rocket Launch(v1.5): Deploy new version to production with enhanced security features` |
|                                   | Spacewalk                  | Swift Patches And Lookout Work, Keeping Systems Extra safe   | Urgent production hotfixes                                | `Spacewalk(Database): Fix critical database connection issue causing downtime` |
|                                   | Space Elevator             | Streamlined Access, Providing Easy Vertical Onboarding, Lifting Entries | Making codebase more accessible                           | `Space Elevator(API): Add new public API endpoints for third-party integrations` |