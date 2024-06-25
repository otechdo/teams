# Installation

```bash
cargo install zuu commiter
```

## Archlinux user install

```bash
paru -S commiter
```

# Cosmic Commit Types

* **Core:**
  * **Star:** New feature or enhancement
  * **Comet:** Bug fix or error resolution
  * **Nebula:** Code refactoring
  * **Pulsar:** Performance improvement
  * **Quasar:** Documentation or clarity improvement
* **Maintenance & Infrastructure:**
  * **Asteroid Belt:** Code cleanup and maintenance
  * **Solar Flare:** Testing-related changes
  * **Dwarf Planet:** Minor updates or fixes
  * **Terraform:** Infrastructure changes
* **Special Events:**
  * **Black Hole:** Removing large chunks of code or features
  * **Wormhole:** Merging branches or connecting code parts
  * **Big Bang:** Initial commit or major feature start
  * **Launch:** Deploying to production or releasing a version
* **Communication & Collaboration:**
  * **Lightspeed:** Significant performance improvements
  * **Mission Control:** Project management changes
  * **Spacewalk:** Urgent hotfixes
  * **Moon Landing:** Major milestone or goal completion
  * **First Contact:** Initial integrations with external systems
  * **Interstellar Communication:** Improving documentation or communication
* **Celestial Events:**
  * **Solar Eclipse:** Temporarily masking functionality
  * **Supernova:** Major, transformative change
  * **Meteor Shower:** Series of small changes or fixes
  * **Solar Wind:** Refactoring code structure
  * **Lunar Eclipse:** Temporarily disabling a feature
  * **Cosmic Dawn:** Initial implementation of a feature
  * **Solar Storm:** Rapid, impactful changes
  * **Lunar Transit:** Minor, temporary change
  * **Perihelion:** Indicates a commit that brings the project closer to its goals or objectives, like a celestial
    body reaching its closest point to the sun in its orbit. This could be a significant milestone or a major
    feature completion.
  * **Aphelion:** Represents a commit that takes the project further away from its immediate goals, but is
    necessary for long-term progress, like a celestial body reaching its farthest point from the sun in its orbit.
    This could be a refactor, a dependency update, or a change in architecture.
* **Celestial Objects:**
  * **White Dwarf:** Improving code comments or documentation
  * **Red Giant:** Expanding a feature or functionality
  * **Neutron Star:** Optimizing code for performance
  * **Binary Star:** Merging features or components
  * **Brown Dwarf:** Undeveloped feature with potential
  * **Quark Star:** Experimental or speculative change
  * **Rogue Planet:** Independent change
  * **Stellar Nursery:** Creation of new components
  * **Planetary Nebula:** Removal or deprecation of a component
  * **Globular Cluster:** Collection of related changes
  * **Void:** Removal of a module, component, or feature
* **Astronomical Concepts:**
  * **Gravity:** Resolving merge conflicts or dependencies
  * **Dark Matter:** Fixing unknown or mysterious bugs
  * **Time Dilation:** Improving code performance
  * **Spacetime:** Changes to date, time, or scheduling
  * **Gravitational Lensing:** Altering data or information flow
  * **Cosmic String:** Connecting code parts
  * **Quantum Fluctuation:** Small, random change
  * **Hawking Radiation:** Removing technical debt
  * **Quantum Entanglement:** Establishing close relationships between code parts
  * **Gravitational Redshift:** Slowing down or reducing code performance
* **Space Exploration:**
  * **Space Probe:** Testing new features or technologies
  * **Station:** Creating or improving environments
  * **Rocket Launch:** Deploying to production
  * **Spacewalk:** Urgent production hotfixes
  * **Space Elevator:** Making codebase more accessible
  * **Warp Drive:** Significant speed improvement
  * **Dyson Sphere:** Comprehensive optimization of a specific area
  * **Generation Ship:** Long-term project for a self-sustaining system
  * **Lagrange Point:** Stabilizing or balancing code parts
  * **Orbital Maneuver:** Changing project direction
  * **Mission Control:** Represents project management-related changes, such as defining processes or updating the
    roadmap. It's about keeping the project on course and ensuring smooth operation.
  * **Moon Landing:** Celebrates the completion of major milestones or significant achievements in the project,
    marking a giant leap forward.
  * **First Contact:** Indicates the initial establishment of connections or integrations with external
    systems orAPIs,opening up new possibilities for communication and data exchange.
  * **Interstellar Travel:** Migration to a new architecture or language.
  * **Rover:** Exploration of new technologies or approaches.
* **Astrophysical Concepts**
  * **Singularity:** Resolution of a complex or hard-to-reproduce issue.
  * **Relativity:** Changes related to time, dates, or timestamps.
  * **Expansion:** Scaling up the system or increasing capacity.
  * **Big Crunch:** Reduction of codebase size or removal of features.

# Commit Message Format

```
<type>(<scope>): <short summary>

[Optional body with more details]

[Optional footer(s)]
```

# Why use it ?

**1. Team Adoption:**

* **Discuss and Agree:** Initiate a conversation with your team about using cosmic commit types. Explain the benefits,
  share this comprehensive guide, and gather feedback.
* **Customize:** Collaboratively decide on the specific commit types you want to use. You can start with the
  comprehensive list provided here and tailor it to your project's specific needs and preferences.
* **Document:** Create a clear and concise reference document outlining the chosen commit types, their meanings, and
  examples. Make this document easily accessible to all team members.

**2. Implementation:**

* **Manual Approach:** You can start using cosmic commit types manually by simply adhering to
  the `<type>(<scope>): <short summary>` format in your commit messages.
* **Git Commit Template:** Create a Git commit template file (e.g., `.gitmessage`) to automatically populate the commit
  message format in your editor. This can help enforce consistency and remind contributors of the available commit
  types.
* **Git Hooks:** Utilize Git hooks, like the `prepare-commit-msg` hook, to validate your commit messages and ensure they
  conform to the chosen format.
* **Automated Tools:** Consider leveraging tools like `commitizen` or `cz-cli` that provide interactive prompts for
  creating commit messages according to your chosen convention. These tools can streamline the process and enforce
  consistency across your team.

**3. Continuous Improvement:**

* **Regular Review:** Periodically review your team's commit history to ensure consistent usage of the cosmic commit
  types and identify any areas where the format could be refined or improved.
* **Feedback Loop:** Encourage open communication and feedback from your team members about the effectiveness of the
  chosen commit types and any suggestions for improvement.
* **Iterative Refinement:** Don't be afraid to experiment and adapt the commit types to better suit your evolving
  project needs. The key is to find a system that works well for your team and enhances your Git workflow.

**4. Continuous Improvement:**

* **Encourage Creativity:** While maintaining consistency, allow team members to add their own flair and personality to
  the commit messages within the established framework.
* **Celebrate Milestones:** Use special event commit types like "Moon Landing" to celebrate significant achievements and
  keep your team motivated.
* **Integration with Other Tools:** Explore integration options with your issue tracking system, CI/CD pipeline, or
  documentation tools to automate processes and maximize the benefits of using cosmic commit types.

By embracing this comprehensive guide and incorporating cosmic commit types into your Git workflow, you can transform
your commit history into a vibrant, informative, and enjoyable reflection of your project's journey. 🚀

## **Improved Clarity and Readability**

* **Descriptive Labels:** The descriptive nature of cosmic commit types makes it instantly clear what kind of change a
  commit introduces (e.g., a new feature, bug fix, or refactor).
* **Visual Scanning:** The use of emojis or keywords like "Star" or "Comet" allows for quick visual scanning of the
  commit history, making it easier to identify specific types of changes.
* **Enhanced Context:** The optional scope field provides additional context about the area of the codebase affected by
  the commit, further aiding understanding.

## **Better Organization and Maintainability**

* **Structured Format:** The consistent format of `<type>(<scope>): <short summary>` ensures a standardized approach to
  commit messages, making them easier to parse and filter.
* **Streamlined History:**  A well-organized commit history makes it easier to track the evolution of the project,
  identify patterns, and pinpoint the introduction of specific changes.
* **Easier Collaboration:**  A clear and consistent commit history facilitates collaboration among team members, as
  everyone understands the meaning and purpose of each commit.

## **Increased Engagement and Fun**

* **Creative Expression:** The cosmic theme adds a touch of personality and fun to the otherwise mundane task of writing
  commit messages.
* **Team Building:** Using a shared vocabulary of commit types can foster a sense of camaraderie and shared purpose
  within the team.
* **Motivation:** The playful nature of these commit types can make the development process more enjoyable and engaging.

## **Additional Benefits**

* **Automated Tools:** Some tools can leverage these commit types to automate tasks like generating changelogs, release
  notes, or even identifying potential issues based on commit patterns.
* **Improved Workflow:** By adopting a structured approach to commit messages, you can streamline your development
  workflow and make it more efficient.
* **Professionalism:**  A well-maintained commit history reflects a professional approach to software development and
  can be a valuable asset when onboarding new team members or showcasing your work.

Overall, using cosmic commit types can enhance your Git workflow, making it more informative, organized, and enjoyable.
It's a simple yet effective way to improve communication and collaboration within your team while adding a touch of
creativity and fun to your daily coding routine.