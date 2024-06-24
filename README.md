# What is it ?

It's a git commit helper with format, check source code quality before commit with commit message like AngularJS.

## Remote

Call your remote `origin`.

## Installation

```bash
cargo install zuu commiter
```

## Usage

```bash
commiter
```

### Commit Message Format

```
<type>(<scope>): <short summary>
  │       │             │
  │       │             └─  Summary in present tense. Not capitalized. No period at the end.
  │       │   
  │       └─ Commit Scope: The scope of the commit free
  │
  └─ Commit Type    Star|Comet|Nebula|Pulsar|Quasar|Asteroid Belt|Solar Flare|Dwarf Planet|
                    Supermassive Black Hole|Eclipse|Supernova|Red Giant|White Dwarf|
                    Neutron Star|Black Hole|Wormhole|Solar Wind|Lunar Eclipse|Cosmic Microwave Background|
                    Gravitational Wave|Event Horizon|Big Bang|Launch|Probe|Slingshot Maneuver|Wormhole|
                    Lightspeed|Mission Control|Spacewalk|Moon Landing|First Contact|Terraform|
                    Interstellar Communication
```

#### Commits examples

Commit example with cosmos type.

##### Star

On introducing significant new features.

**General Features:**

* **Star:** Introduce a customizable dashboard for personalized user experiences.
* **Star:** Unleash a dynamic search bar with advanced filtering options.
* **Star:** Enable real-time collaboration for seamless teamwork.
* **Star:** Launch a personalized recommendation system based on user preferences.

**Specific Feature Areas:**

* **Star (Payments):** Integrate a secure payment gateway for seamless transactions.
* **Star (Notifications):** Introduce a customizable notification system for enhanced user engagement.
* **Star (Analytics):** Add an in-depth analytics dashboard for data-driven decision-making.
* **Star (Social):** Implement social sharing buttons to expand content reach.

**Descriptive Star Commits:**

* **Star:** Implement a new user onboarding flow to improve user activation.
* **Star:** Add a dark mode option to enhance user experience in low-light environments.
* **Star:** Integrate a powerful WYSIWYG editor for creating rich text content.
* **Star:** Launch a gamification system to incentivize user engagement and retention.

**Additional Notes:**

* These commit messages prioritize clarity and conciseness, focusing on the core value or impact of the new feature.
* They adhere to the conventional commit format, starting with the type ("Star") and followed by a brief description.
* Remember to tailor your commit messages to your specific project and audience.

##### Comet

On resolving unexpected issues or bugs.

General Fixes:

* **Comet:** Resolve unexpected crash on login page.
* **Comet:** Fix intermittent 500 error on API endpoint.
* **Comet:** Address memory leak in image processing module.
* **Comet:** Patch security vulnerability in user authentication flow.

Specific Bug Fixes:

* **Comet (UI):** Fix misaligned elements in responsive layout.
* **Comet (API):** Correct incorrect data returned in search results.
* **Comet (Database):** Resolve data inconsistency causing duplicate entries.
* **Comet (Performance):** Eliminate bottleneck in background task execution.

Descriptive Comet Commits:

* **Comet:** Fix issue where user data was not being saved correctly.
* **Comet:** Address race condition causing occasional server crashes.
* **Comet:** Resolve issue with incorrect calculations in financial reports.
* **Comet:** Patch vulnerability allowing unauthorized access to sensitive data.

Additional Notes:

* These commit messages prioritize clarity and conciseness, focusing on the specific issue or bug being fixed.
* They adhere to the conventional commit format, starting with the type ("Comet") and followed by a brief description.
* Remember to tailor your commit messages to your specific project and audience.

##### Nebula

On refactoring and restructuring code.

**General Refactoring:**

* **Nebula:** Restructure component hierarchy for better maintainability.
* **Nebula:** Consolidate redundant functions into reusable modules.
* **Nebula:** Simplify complex logic for improved readability.
* **Nebula:** Extract common patterns into shared utilities.
* **Nebula:** Improve code organization and modularity.

**Specific Refactoring Areas:**

* **Nebula (UI):** Refactor CSS for consistent styling and maintainability.
* **Nebula (API):** Redesign API endpoints for better consistency and error handling.
* **Nebula (Database):** Optimize database schema for improved performance.
* **Nebula (Performance):** Refactor critical code paths for enhanced efficiency.

**Descriptive Nebula Commits:**

* **Nebula:** Replace inheritance with composition for greater flexibility.
* **Nebula:** Break down monolithic functions into smaller, focused units.
* **Nebula:** Apply design patterns to improve code structure and scalability.
* **Nebula:** Upgrade to newer language features for cleaner syntax and functionality.

**Additional Notes:**

* These commit messages focus on the intent and impact of the refactoring, rather than the specific code changes.
* They adhere to the conventional commit format, starting with the type ("Nebula") and followed by a brief description.
* Remember to tailor your commit messages to your specific project and audience.

##### Pulsar

On performance improvements.

**General Performance:**

* **Pulsar:** Significantly reduced latency in API response times.
* **Pulsar:** Optimized database queries for faster data retrieval.
* **Pulsar:** Implemented caching mechanism to reduce server load.
* **Pulsar:** Refactored critical code paths for enhanced efficiency.

**Specific Performance Areas:**

* **Pulsar (Network):** Minimized network requests for faster page loads.
* **Pulsar (Memory):** Optimized memory usage by reducing object allocation.
* **Pulsar (CPU):** Improved CPU utilization by parallelizing computations.
* **Pulsar (Rendering):** Optimized rendering pipeline for smoother animations.

**Descriptive Pulsar Commits:**

* **Pulsar:** Upgraded algorithm for faster sorting of large datasets.
* **Pulsar:** Introduced lazy loading to defer image loading until needed.
* **Pulsar:** Replaced inefficient data structure with a more performant alternative.
* **Pulsar:** Fine-tuned database indexes for quicker query execution.

**Additional Notes:**

* These commit messages focus on the specific performance improvements achieved, using clear and concise language.
* They adhere to the conventional commit format, starting with the type ("Pulsar") and followed by a brief description.

##### Quasar

On improvements to documentation and clarity.

**General Documentation:**

* **Quasar:** Updated API documentation with new endpoints and parameters.
* **Quasar:** Added comprehensive README file with installation and usage instructions.
* **Quasar:** Created a detailed guide for troubleshooting common errors and issues.
* **Quasar:** Expanded code comments to clarify complex logic and algorithms.

**Specific Documentation Areas:**

* **Quasar (API):** Improved descriptions of request/response formats and error codes.
* **Quasar (Tutorial):** Wrote a step-by-step tutorial for beginners.
* **Quasar (Reference):** Added a reference guide for available configuration options.
* **Quasar (Examples):** Provided working code examples for common use cases.

**Descriptive Quasar Commits:**

* **Quasar:** Clarified ambiguous variable names and function signatures.
* **Quasar:** Added diagrams and illustrations to explain complex concepts.
* **Quasar:** Refactored documentation structure for better navigation and readability.
* **Quasar:** Translated documentation into multiple languages for wider accessibility.

**Additional Notes:**

* These commit messages prioritize clarity and conciseness, focusing on the specific improvements made to the
  documentation.
* They adhere to the conventional commit format, starting with the type ("Quasar") and followed by a brief description.

##### Asteroid Belt

On code cleanup, organization, and maintenance.

**General Maintenance:**

* **Asteroid Belt:** Removed unused imports and variables.
* **Asteroid Belt:** Organized project files and folders.
* **Asteroid Belt:** Updated dependencies to latest versions.
* **Asteroid Belt:** Refactored code for consistency and readability.

**Specific Cleanup Areas:**

* **Asteroid Belt (UI):** Removed unused CSS styles and classes.
* **Asteroid Belt (API):** Consolidated redundant API endpoints.
* **Asteroid Belt (Database):** Removed unused tables and columns.
* **Asteroid Belt (Testing):** Refactored and optimized test suite.

**Descriptive Asteroid Belt Commits:**

* **Asteroid Belt:** Cleaned up commented-out code and debugging statements.
* **Asteroid Belt:** Standardized code formatting according to project guidelines.
* **Asteroid Belt:** Removed duplicate code and logic.
* **Asteroid Belt:** Optimized build scripts for faster execution.

**Additional Notes:**

* These commit messages prioritize clarity and conciseness, focusing on the specific cleanup or maintenance tasks
  performed.
* They adhere to the conventional commit format, starting with the type ("Asteroid Belt") and followed by a brief
  description.

##### Solar Flare

On adding or updating tests.

General Testing:

* **Solar Flare:** Added comprehensive unit tests for new user authentication module.
* **Solar Flare:** Expanded test coverage for core business logic.
* **Solar Flare:** Introduced integration tests to verify API endpoint interactions.
* **Solar Flare:** Implemented end-to-end tests for critical user flows.

Specific Testing Areas:

* **Solar Flare (UI):** Added UI tests to cover form validation and submission.
* **Solar Flare (API):** Created test suite for error handling and edge cases in API responses.
* **Solar Flare (Database):** Wrote tests to ensure data integrity and consistency.
* **Solar Flare (Performance):** Added stress tests to evaluate system performance under load.

Descriptive Solar Flare Commits:

* **Solar Flare:** Refactored existing tests for better maintainability and readability.
* **Solar Flare:** Fixed flaky tests and improved overall test stability.
* **Solar Flare:** Upgraded testing framework to the latest version.
* **Solar Flare:** Introduced mocking for external dependencies in unit tests.

Additional Notes:

* These commit messages prioritize clarity and conciseness, focusing on the specific testing activities performed.
* They adhere to the conventional commit format, starting with the type ("Solar Flare") and followed by a brief
  description.
* Remember to tailor your commit messages to your specific project and audience.

##### Dwarf Planet

Small but essential maintenance tasks and updates.

General Maintenance:

* **Dwarf Planet:** Update dependencies to latest stable versions.
* **Dwarf Planet:** Refactor minor code formatting and styling inconsistencies.
* **Dwarf Planet:** Improve error handling in logging module.
* **Dwarf Planet:** Update copyright year in project files.

Specific Maintenance Areas:

* **Dwarf Planet (CI):** Refine continuous integration pipeline for faster builds.
* **Dwarf Planet (Docs):** Fix broken links and typos in documentation.
* **Dwarf Planet (Config):** Update configuration settings for improved security.
* **Dwarf Planet (Tests):** Refactor test data for better maintainability.

Descriptive Dwarf Planet Commits:

* **Dwarf Planet:** Remove unused assets from project directory.
* **Dwarf Planet:** Add missing semicolons for code consistency.
* **Dwarf Planet:** Update project roadmap and release notes.
* **Dwarf Planet:** Refine wording in user interface for better clarity.

Additional Notes:

* These commit messages prioritize clarity and conciseness, focusing on the specific maintenance or update tasks
  performed.
* They adhere to the conventional commit format, starting with the type ("Dwarf Planet") and followed by a brief
  description.
* Remember to tailor your commit messages to your specific project and audience.

##### Black Hole

Removing unused code, files or dependencies.

General Cleanup:

* **Black Hole:** Devoured unused CSS rules and classes.
* **Black Hole:** Sucked in commented-out code and debug statements.
* **Black Hole:** Consumed redundant libraries and dependencies.
* **Black Hole:** Eradicated dead code paths and unused functions.

Specific Cleanup Areas:

* **Black Hole (UI):** Removed unused image assets and icons.
* **Black Hole (API):** Deprecated and removed unused API endpoints.
* **Black Hole (Database):** Dropped obsolete tables and columns.
* **Black Hole (Testing):** Deleted outdated or redundant test cases.

Descriptive Black Hole Commits:

* **Black Hole:** Eliminated feature flags for abandoned experiments.
* **Black Hole:** Removed legacy code that is no longer supported.
* **Black Hole:** Consumed unused configuration settings and environment variables.
* **Black Hole:** Removed references to deprecated third-party services.

Additional Notes:

* These commit messages use the "Black Hole" metaphor to emphasize the removal of unnecessary elements, making the
  codebase leaner and more efficient.
* They adhere to the conventional commit format, starting with the type ("Black Hole") and followed by a brief
  description.

##### Wormhole

On merging branches or connecting disparate parts of the codebase.

**General Merging and Integration:**

* **Wormhole:** Merge feature branch 'user-profile' into 'develop'.
* **Wormhole:** Successfully integrated third-party payment gateway.
* **Wormhole:** Connected frontend components to backend API endpoints.
* **Wormhole:** Bridged the gap between legacy code and new architecture.

**Specific Wormhole Commits:**

* **Wormhole (UI):** Merged design updates from Figma into React components.
* **Wormhole (API):** Integrated new authentication service with existing endpoints.
* **Wormhole (Database):** Migrated data from legacy system to new database schema.
* **Wormhole (Testing):** Connected unit tests with continuous integration pipeline.

**Descriptive Wormhole Commits:**

* **Wormhole:** Established seamless communication between frontend and backend.
* **Wormhole:** Unified data models across multiple microservices.
* **Wormhole:** Created a bridge between two previously incompatible modules.
* **Wormhole:** Integrated real-time updates between client and server.

**Humorous Wormhole Commits:**

* **Wormhole:** "Jumped through hyperspace and merged these branches!"
* **Wormhole:** "Created a shortcut through the codebase like a cosmic wormhole."
* **Wormhole:** "This merge is so smooth, it's like traveling through a wormhole!"

**Additional Notes:**

* The "Wormhole" commit type is best suited for commits that involve merging branches, integrating different parts of
  the codebase, or connecting with external systems.
* You can combine it with other commit types for more specific information (e.g., "feat(Wormhole): Integrate new search
  API with existing UI").
* Feel free to add your own creative flair to the commit messages!

##### Solar Wind

On refactoring code for a smoother flow and improved maintainability.

**General Refactoring:**

* **Solar Wind:** Streamlined user authentication process for a smoother login experience.
* **Solar Wind:** Removed redundant code and optimized data flow for faster execution.
* **Solar Wind:** Improved readability and consistency across components.
* **Solar Wind:** Simplified complex logic for better maintainability.

**Specific Refactoring Areas:**

* **Solar Wind (UI):** Consolidated styling rules and removed unused components.
* **Solar Wind (API):** Standardized error handling and response formats across endpoints.
* **Solar Wind (Database):** Refactored queries for better performance and readability.
* **Solar Wind (Testing):** Cleaned up test code and improved test organization.

**Descriptive Solar Wind Commits:**

* **Solar Wind:** Replaced callback hell with async/await for improved code clarity.
* **Solar Wind:** Extracted common functionality into reusable helper functions.
* **Solar Wind:** Applied design patterns to improve code structure and scalability.
* **Solar Wind:** Upgraded to newer language features for more concise syntax.

**Additional Notes:**

* These commit messages focus on the positive impact of refactoring, such as improved readability, maintainability, and
  performance.
* They adhere to the conventional commit format, starting with the type ("Solar Wind") and followed by a brief
  description.

##### Lunar Eclipse

On temporarily disabling or hiding features.

**General Feature Disabling:**

* **Lunar Eclipse:** Temporarily disable new user onboarding flow for maintenance.
* **Lunar Eclipse:** Hide experimental feature flag until further testing.
* **Lunar Eclipse:** Disable social sharing buttons due to security concerns.
* **Lunar Eclipse:** Turn off analytics tracking during development phase.

**Specific Feature Disabling:**

* **Lunar Eclipse (UI):** Disable dark mode toggle due to rendering issues.
* **Lunar Eclipse (API):** Temporarily block access to sensitive endpoint.
* **Lunar Eclipse (Payments):** Disable specific payment method for maintenance.
* **Lunar Eclipse (Notifications):** Mute non-essential notifications for testing.

**Descriptive Lunar Eclipse Commits:**

* **Lunar Eclipse:** Hide new UI element until translations are complete.
* **Lunar Eclipse:** Temporarily disable beta feature due to unexpected errors.
* **Lunar Eclipse:** Suspend background task for performance optimization.
* **Lunar Eclipse:** Deactivate user accounts with suspicious activity.

**Humorous Lunar Eclipse Commits:**

* **Lunar Eclipse:** "The feature is taking a break to admire the moon."
* **Lunar Eclipse:** "This functionality is hiding in the shadows for now."
* **Lunar Eclipse:** "Putting this feature to sleep until it's fully charged."

**Additional Notes:**

* These commit messages prioritize clarity and conciseness, focusing on the specific feature being disabled and the
  reason for it.
* They adhere to the conventional commit format, starting with the type ("Lunar Eclipse") and followed by a brief
  description.

##### Cosmic Microwave Background

On adding logging, monitoring, or observability to your code.

**General Observability:**

* **Cosmic Microwave Background:** Added comprehensive logging for API requests and responses.
* **Cosmic Microwave Background:** Integrated error tracking and reporting system.
* **Cosmic Microwave Background:** Implemented real-time performance monitoring dashboard.
* **Cosmic Microwave Background:** Introduced tracing for distributed transactions.

**Specific Monitoring Areas:**

* **Cosmic Microwave Background (Database):** Added query performance logging and analysis.
* **Cosmic Microwave Background (Network):** Set up monitoring for latency and throughput.
* **Cosmic Microwave Background (Memory):** Implemented memory leak detection and alerts.
* **Cosmic Microwave Background (Security):** Added intrusion detection and logging mechanisms.

**Descriptive Cosmic Microwave Background Commits:**

* **Cosmic Microwave Background:** Captured detailed metrics for user behavior analysis.
* **Cosmic Microwave Background:** Enabled distributed tracing for improved debugging.
* **Cosmic Microwave Background:** Configured alerts for critical system events.
* **Cosmic Microwave Background:** Visualized system health and performance metrics on a dashboard.

**Humorous Cosmic Microwave Background Commits:**

* **Cosmic Microwave Background:** "Tuning in to the whispers of the universe (aka logs)."
* **Cosmic Microwave Background:** "Shedding light on the darkest corners of the codebase."
* **Cosmic Microwave Background:** "Listening to the heartbeat of the application."

**Additional Notes:**

* These commit messages prioritize clarity and conciseness, focusing on the specific observability or monitoring
  improvements implemented.
* They adhere to the conventional commit format, starting with the type ("Cosmic Microwave Background") and followed by
  a brief description.

##### Gravitational Wave

On fixing subtle but impactful bugs or making small but significant improvements.

**General Fixes and Improvements:**

* **Gravitational Wave:** Fixed a rounding error that was causing discrepancies in financial calculations.
* **Gravitational Wave:** Adjusted algorithm parameters for improved accuracy and efficiency.
* **Gravitational Wave:** Smoothed out minor UI glitches for a more polished user experience.
* **Gravitational Wave:** Fine-tuned database indexes for better query performance.

**Specific Gravitational Wave Commits:**

* **Gravitational Wave (Security):** Tightened up input validation to prevent potential attacks.
* **Gravitational Wave (Localization):** Fixed minor translation errors in multiple languages.
* **Gravitational Wave (Performance):** Optimized memory usage in a critical loop.
* **Gravitational Wave (Accessibility):** Improved keyboard navigation for screen reader users.

**Descriptive Gravitational Wave Commits:**

* **Gravitational Wave:** Addressed edge case where button was not responding to clicks.
* **Gravitational Wave:** Corrected minor timing issue in animation sequence.
* **Gravitational Wave:** Fixed typo in error message that was confusing users.
* **Gravitational Wave:** Improved logging to capture more detailed information about errors.

**Humorous Gravitational Wave Commits:**

* **Gravitational Wave:** "This tiny change caused a ripple effect of improvements."
* **Gravitational Wave:** "Sent out a gravitational wave to fix that pesky bug."
* **Gravitational Wave:** "Even the smallest changes can have a big impact, like a gravitational wave."

**Additional Notes:**

* The "Gravitational Wave" commit type is best suited for commits that address seemingly small issues that could have a
  significant impact on the overall system.
* You can combine it with other commit types for more specific information (e.g., "fix(Gravitational Wave): Corrected
  off-by-one error in pagination logic").

##### Event Horizon

On significant changes that mark a point of no return.

**Major Changes and Decisions:**

* **Event Horizon:** Removed support for legacy browser versions.
* **Event Horizon:** Switched to a new database technology.
* **Event Horizon:** Deployed major API version update.
* **Event Horizon:** Removed deprecated features and functionality.
* **Event Horizon:** Changed project license.

**Specific Event Horizon Commits:**

* **Event Horizon (Security):** Enforced stricter security policies with no backward compatibility.
* **Event Horizon (Architecture):** Migrated from monolithic to microservices architecture.
* **Event Horizon (Data):** Permanently deleted obsolete user data.
* **Event Horizon (Infrastructure):** Switched cloud providers.

**Descriptive Event Horizon Commits:**

* **Event Horizon:** Dropped support for Node.js v12 and below.
* **Event Horizon:** Replaced custom authentication with OAuth2 provider.
* **Event Horizon:** Removed support for legacy payment gateway.
* **Event Horizon:** Switched to a new frontend framework (e.g., React to Vue).

**Humorous Event Horizon Commits:**

* **Event Horizon:** "There's no turning back now!"
* **Event Horizon:** "This change is like crossing the Rubicon."
* **Event Horizon:** "We've entered a new era for this project."

**Additional Notes:**

* The "Event Horizon" commit type is best suited for commits that have significant consequences and cannot be easily
  reversed.
* It adds a sense of gravity and importance to these changes.
* Remember to communicate these major changes clearly to your team and stakeholders.

##### Big Bang

On the initial creation or setup of different aspects of a project.

**General Project Initialization:**

* **Big Bang:** Initial commit: Creating the foundation for a new project.
* **Big Bang:** Setting up the project structure and basic configuration.
* **Big Bang:** Laying the groundwork for a new adventure in code.

**Specific Component or Feature Initialization:**

* **Big Bang (Auth):** Initial setup for user authentication and authorization.
* **Big Bang (API):** First draft of API endpoints and data models.
* **Big Bang (UI):** Basic layout and styling for the landing page.
* **Big Bang (Database):** Initial database schema design and migrations.
* **Big Bang (Testing):** Setting up testing framework and initial unit tests.

**Descriptive Big Bang Commits:**

* **Big Bang:** Create a new repository and initialize Git.
* **Big Bang:** Add essential dependencies and libraries.
* **Big Bang:** Set up basic project scaffolding and folder structure.
* **Big Bang:** Write initial documentation and README file.

**Humorous Big Bang Commits:**

* **Big Bang:** Let there be code!
* **Big Bang:** And so it begins...
* **Big Bang:** In the beginning, there was nothing... then there was this commit.

**Additional Notes:**

* The "Big Bang" commit type is most appropriate for the very first commit in a repository or the first commit related
  to a new feature or component.
* It can be combined with other commit types for more specific information (e.g., "Big Bang (feat): Initial
  implementation of user profiles").

##### Launch

On deploying code or releasing features.

**General Deployment:**

* **Launch:** Deploy v1.2.0 to production environment
* **Launch:** Release new user interface with enhanced features
* **Launch:** Roll out performance improvements for faster loading times
* **Launch:** Push critical bug fixes to live servers

**Specific Launch Commits:**

* **Launch (Web):** Deploy updated website with responsive design.
* **Launch (Mobile):** Release new version of mobile app on app stores.
* **Launch (API):** Update API with new endpoints and documentation.
* **Launch (Backend):** Deploy optimized backend services for improved scalability.

**Descriptive Launch Commits:**

* **Launch:** Successfully deployed new payment gateway integration.
* **Launch:** Rolled out A/B testing for new landing page design.
* **Launch:** Activated new user registration and onboarding flow.
* **Launch:** Released new version with support for multiple languages.

**Humorous Launch Commits:**

* **Launch:** "Houston, we have liftoff!"
* **Launch:** "3, 2, 1... Blast off!"
* **Launch:** "To infinity and beyond!"

**Additional Notes:**

* The "Launch" commit type is best suited for commits that involve deploying code to a live environment or releasing new
  features to users.
* You can combine it with other commit types for more specific information (e.g., "Launch (feat): Release new user
  dashboard").

##### Probe

On experimental features or exploratory work.

**General Experimentation:**

* **Probe:** Explore new machine learning algorithm for recommendation engine.
* **Probe:** Implement prototype for real-time chat feature.
* **Probe:** Investigate performance optimizations for image processing.
* **Probe:** Test alternative data storage solutions for scalability.

**Specific Probe Commits:**

* **Probe (UI):** Experiment with different layouts for product listings.
* **Probe (API):** Test new authentication method using JWT tokens.
* **Probe (Database):** Evaluate NoSQL database for improved performance.
* **Probe (Deployment):** Experiment with serverless deployment for faster scaling.

**Descriptive Probe Commits:**

* **Probe:** Prototype voice command integration for accessibility.
* **Probe:** Investigate potential security vulnerabilities in user input handling.
* **Probe:** Experiment with using WebAssembly to improve performance.
* **Probe:** Test new data visualization techniques for dashboard.

**Humorous Probe Commits:**

* **Probe:** "Boldly going where no code has gone before."
* **Probe:** "Exploring strange new worlds of functionality."
* **Probe:** "Sending out a probe to gather data on user behavior."

**Additional Notes:**

* The "Probe" commit type is best suited for commits that involve experimentation, prototyping, or exploring new ideas.
* It indicates that the changes are not final and may be subject to further refinement or revision.
* Remember to communicate the experimental nature of these changes to your team and stakeholders.

##### Slingshot Maneuver

On reverting previous commits or changes.

**General Reverts:**

* **Slingshot Maneuver:** Revert accidental deletion of critical files.
* **Slingshot Maneuver:** Undo unintended changes to database schema.
* **Slingshot Maneuver:** Roll back buggy feature release to stable version.
* **Slingshot Maneuver:** Revert to previous commit due to unexpected side effects.

**Specific Slingshot Maneuver Commits:**

* **Slingshot Maneuver (UI):** Revert breaking changes to navigation menu.
* **Slingshot Maneuver (API):** Undo incompatible changes to endpoint parameters.
* **Slingshot Maneuver (Performance):** Revert optimization that caused regression.
* **Slingshot Maneuver (Security):** Roll back changes that introduced a vulnerability.

**Descriptive Slingshot Maneuver Commits:**

* **Slingshot Maneuver:** Revert commit [hash] due to conflicts with other changes.
* **Slingshot Maneuver:** Undo accidental merge of incorrect branch.
* **Slingshot Maneuver:** Backtrack to a stable state after failed experiment.
* **Slingshot Maneuver:** Restore previous configuration settings after incorrect update.

**Humorous Slingshot Maneuver Commits:**

* **Slingshot Maneuver:** "Oops, I did it again!" (Britney Spears reference)
* **Slingshot Maneuver:** "Undoing my best impression of a black hole."
* **Slingshot Maneuver:** "Taking a cosmic U-turn back to safety."
* **Slingshot Maneuver:** "Abort mission! Abort mission!"

**Additional Notes:**

* The "Slingshot Maneuver" commit type is best suited for commits that undo previous changes, either due to errors,
  unintended consequences, or failed experiments.
* It emphasizes the act of quickly reversing course to avoid further problems.
* Remember to document the reason for the revert in the commit message for future reference.

##### Wormhole

On merging branches or connecting disparate parts of the codebase.

**General Merging and Integration:**

* Wormhole: Merge feature branch 'user-authentication' into 'main'.
* Wormhole: Integrate third-party payment gateway with existing checkout flow.
* Wormhole: Connect backend API to frontend components for real-time data fetching.
* Wormhole: Bridge the gap between legacy system and new microservice architecture.

**Specific Wormhole Commits:**

* Wormhole (UI): Integrate new design system with existing components.
* Wormhole (API): Merge authentication middleware into all endpoints.
* Wormhole (Database): Connect new database cluster to existing application.
* Wormhole (Testing): Integrate unit tests with continuous integration pipeline.

**Descriptive Wormhole Commits:**

* Wormhole: Established seamless communication between frontend and backend using WebSockets.
* Wormhole: Unified data models across multiple microservices using a shared library.
* Wormhole: Created a bridge between legacy codebase and modern framework.
* Wormhole: Implemented a data pipeline to transfer information between two separate systems.

**Humorous Wormhole Commits:**

* Wormhole: "Jumped through hyperspace and merged these branches!"
* Wormhole: "Created a shortcut through the codebase like a cosmic wormhole."
* Wormhole: "This merge was so smooth, it felt like traveling through a wormhole!"
* Wormhole: "Finally bridged the gap between these two worlds (of code)."

**Additional Notes:**

* The "Wormhole" commit type is best suited for commits that involve merging branches, integrating different parts of
  the codebase, or connecting with external systems.
* You can combine it with other commit types for more specific information (e.g., "feat(Wormhole): Integrate new search
  API with existing UI").
* Feel free to add your own creative flair to the commit messages!

##### Lightspeed

On significant performance improvements.

**General Performance Improvements:**

* **Lightspeed:** Improved page load time by 30% through asset optimization.
* **Lightspeed:** Reduced database query time by implementing caching mechanisms.
* **Lightspeed:** Enhanced API response speed by optimizing server-side logic.
* **Lightspeed:** Optimized rendering pipeline for smoother UI interactions.

**Specific Lightspeed Commits:**

* **Lightspeed (Backend):** Implemented asynchronous processing for faster data handling.
* **Lightspeed (Frontend):** Minified and bundled JavaScript files for reduced network latency.
* **Lightspeed (Database):** Added indexes and optimized query execution plans.
* **Lightspeed (Algorithm):** Replaced inefficient algorithm with a more performant alternative.

**Descriptive Lightspeed Commits:**

* **Lightspeed:** Upgraded image compression algorithm for faster downloads.
* **Lightspeed:** Introduced lazy loading for images below the fold.
* **Lightspeed:** Eliminated unnecessary computations in critical code paths.
* **Lightspeed:** Improved caching strategy for frequently accessed data.

**Humorous Lightspeed Commits:**

* **Lightspeed:** "Made the code faster than a tachyon burst!"
* **Lightspeed:** "This code is now so fast, it's breaking the time barrier!"
* **Lightspeed:** "Engaged warp drive for maximum performance!"
* **Lightspeed:** "Accelerated to ludicrous speed!"

**Additional Notes:**

* The "Lightspeed" commit type is best suited for commits that significantly enhance the performance of your code or
  application.
* You can combine it with other commit types for more specific information (e.g., "perf(Lightspeed): Optimized database
  queries for faster response times").

##### Mission Control

On project management and organizational changes.

**General Project Management:**

* **Mission Control:** Updated project roadmap and milestones.
* **Mission Control:** Established new coding conventions and guidelines.
* **Mission Control:** Implemented a new task tracking and management system.
* **Mission Control:** Conducted a comprehensive code review and audit.

**Specific Mission Control Commits:**

* **Mission Control (Agile):** Introduced a new sprint planning and backlog grooming process.
* **Mission Control (Documentation):** Created a comprehensive project wiki for knowledge sharing.
* **Mission Control (Communication):** Established communication channels and protocols for team collaboration.
* **Mission Control (Deployment):** Automated deployment pipeline for faster and more reliable releases.

**Descriptive Mission Control Commits:**

* **Mission Control:** Defined clear roles and responsibilities for team members.
* **Mission Control:** Established code review guidelines and checklists.
* **Mission Control:** Introduced a bug triage process for efficient issue resolution.
* **Mission Control:** Set up automated notifications for critical system events.

**Humorous Mission Control Commits:**

* **Mission Control:** "Houston, we have a plan!"
* **Mission Control:** "Taking the project to new heights!"
* **Mission Control:** "All systems go for a successful launch!"
* **Mission Control:** "Navigating the project through uncharted territories."

**Additional Notes:**

* The "Mission Control" commit type is best suited for commits that involve project management, organization, or process
  improvements.
* You can combine it with other commit types for more specific information (e.g., "Mission Control (docs): Update
  project documentation").

##### Spacewalk

On fixing critical issues in production or performing urgent maintenance.

**General Spacewalk Commits:**

* **Spacewalk:** Hotfixed critical security vulnerability in production environment.
* **Spacewalk:** Resolved major outage caused by database deadlock.
* **Spacewalk:** Patched memory leak causing application crashes.
* **Spacewalk:** Emergency deployment to fix broken authentication flow.

**Specific Spacewalk Commits:**

* **Spacewalk (API):** Fixed invalid response causing client-side errors.
* **Spacewalk (Database):** Repaired corrupted data causing incorrect calculations.
* **Spacewalk (Infrastructure):** Restored server connectivity after unexpected failure.
* **Spacewalk (Security):** Blocked malicious traffic attempting to exploit vulnerability.

**Descriptive Spacewalk Commits:**

* **Spacewalk:** Reverted recent deployment causing unexpected behavior.
* **Spacewalk:** Applied temporary patch to prevent further data loss.
* **Spacewalk:** Increased logging level for faster troubleshooting.
* **Spacewalk:** Disabled problematic feature to restore stability.

**Humorous Spacewalk Commits:**

* **Spacewalk:** "Houston, we had a problem, but it's fixed now!"
* **Spacewalk:** "Donning my spacesuit to tackle this urgent issue."
* **Spacewalk:** "This hotfix is out of this world!"
* **Spacewalk:** "Floating through the codebase to patch this bug."

**Additional Notes:**

* The "Spacewalk" commit type is best suited for commits that address urgent or critical issues in a live production
  environment.
* It conveys a sense of urgency and importance of the fix.
* Be sure to include details about the issue and the resolution in the commit message for future reference.

##### Moon Landing

On completing major milestones or releases.

**General Milestone Completion:**

* **Moon Landing:** Successfully launched version 2.0
* **Moon Landing:** Deployed major overhaul of user interface
* **Moon Landing:** Hit 50,000 active users milestone
* **Moon Landing:** Successfully migrated database to new provider

**Specific Moon Landing Commits:**

* **Moon Landing (Feature):** Integrated real-time collaboration features.
* **Moon Landing (Performance):** Achieved 30% reduction in page load times.
* **Moon Landing (Security):** Implemented two-factor authentication for enhanced security.
* **Moon Landing (Accessibility):** Ensured full compliance with WCAG 2.1 AA standards.

**Descriptive Moon Landing Commits:**

* **Moon Landing:** Completed beta testing and bug fixing phase for new product launch.
* **Moon Landing:** Finalized API documentation and published developer portal.
* **Moon Landing:** Successfully integrated with third-party CRM system.
* **Moon Landing:** Deployed machine learning model for personalized recommendations.

**Humorous Moon Landing Commits:**

* **Moon Landing:** "One small step for code, one giant leap for the app!"
* **Moon Landing:** "This project is over the moon!"
* **Moon Landing:** "We're not just shooting for the moon, we're landing on it!"
* **Moon Landing:** "The eagle has landed... in production!"

**Additional Notes:**

* The "Moon Landing" commit type is best suited for commits that mark significant achievements or milestones in the
  project's development.
* It conveys a sense of accomplishment and progress.

##### First Contact

On establishing initial connections or integrations with external systems or APIs.

**General Integration and Connection:**

* **First Contact:** Established connection with new payment gateway API.
* **First Contact:** Integrated third-party authentication service (e.g., OAuth2).
* **First Contact:** Successfully connected to external data source via API.
* **First Contact:** Implemented webhook integration for real-time updates.

**Specific First Contact Commits:**

* **First Contact (API):** Added initial API client for interacting with external service.
* **First Contact (Auth):** Integrated social login (e.g., Google, Facebook) using OAuth.
* **First Contact (Data):** Imported data from external CSV file into database.
* **First Contact (Analytics):** Integrated analytics platform to track user behavior.

**Descriptive First Contact Commits:**

* **First Contact:** Successfully fetched data from weather API for real-time display.
* **First Contact:** Sent first email notification through integrated email service provider.
* **First Contact:** Connected to real-time messaging service for chat functionality.
* **First Contact:** Received initial webhook notification from external system.

**Humorous First Contact Commits:**

* **First Contact:** "Greetings, Earthlings! We come in peace (and with new features)."
* **First Contact:** "Making friends with a new API - hopefully it speaks our language."
* **First Contact:** "Hello world, we're now connected to the greater universe of data!"
* **First Contact:** "Extending our reach beyond the stars with this integration."

**Additional Notes:**

* The "First Contact" commit type is best suited for commits that establish the initial connection or integration with
  an external system, API, or service.
* It conveys the idea of reaching out and establishing a new link within the broader ecosystem of your project.
* Consider including details about the integrated system or API in the commit message for clarity.

##### Terraform

On infrastructure-as-code (IaC) changes using Terraform.

**General Terraform Commits:**

* **Terraform:** Provision new AWS EC2 instance for web server.
* **Terraform:** Update security group rules to restrict access to database.
* **Terraform:** Create new S3 bucket for storing application logs.
* **Terraform:** Modify load balancer configuration to improve traffic distribution.

**Specific Terraform Commits:**

* **Terraform (AWS):** Deploy new VPC with public and private subnets.
* **Terraform (Azure):** Create Azure Functions for event-driven processing.
* **Terraform (GCP):** Provision Google Kubernetes Engine (GKE) cluster.
* **Terraform (Networking):** Configure DNS records for new domain.

**Descriptive Terraform Commits:**

* **Terraform:** Scale out database cluster to handle increased traffic.
* **Terraform:** Add monitoring and alerting for infrastructure resources.
* **Terraform:** Automate backup and restore process for critical data.
* **Terraform:** Refactor Terraform code for improved readability and maintainability.

**Humorous Terraform Commits:**

* **Terraform:** "Terraforming the cloud landscape, one resource at a time."
* **Terraform:** "Building a digital empire with infrastructure as code."
* **Terraform:** "Unleashing the power of automation to conquer the cloud."
* **Terraform:** "Making infrastructure changes as easy as pie (or JSON)."

**Additional Notes:**

* The "Terraform" commit type is best suited for commits that involve changes to infrastructure code managed by
  Terraform.
* You can combine it with other commit types for more specific information (e.g., "feat(Terraform): Add new S3 bucket
  for storing user data").
* Consider adding resource names or identifiers to your commit messages for better tracking and reference.

##### Interstellar Communication

On improvements to communication, documentation, or collaboration within a project.

**General Communication and Collaboration:**

* **Interstellar Communication:** Improved documentation of API endpoints and parameters.
* **Interstellar Communication:** Streamlined code review process for faster feedback loops.
* **Interstellar Communication:** Implemented real-time chat for team collaboration.
* **Interstellar Communication:** Established clear guidelines for commit messages and branching strategy.

**Specific Interstellar Communication Commits:**

* **Interstellar Communication (Docs):** Added tutorials and examples for new users.
* **Interstellar Communication (Collaboration):** Integrated project management tools for better task tracking.
* **Interstellar Communication (Code Review):** Set up automated code style checks and linters.
* **Interstellar Communication (Knowledge Sharing):** Created a knowledge base for common issues and solutions.

**Descriptive Interstellar Communication Commits:**

* **Interstellar Communication:** Clarified error messages for better user experience.
* **Interstellar Communication:** Improved code comments to enhance readability and understanding.
* **Interstellar Communication:** Established a communication protocol for resolving conflicts.
* **Interstellar Communication:** Created a shared glossary of technical terms.

**Humorous Interstellar Communication Commits:**

* **Interstellar Communication:** "Sending a message in a bottle to future developers."
* **Interstellar Communication:** "Building bridges, not walls, between code modules."
* **Interstellar Communication:** "Deciphering the ancient alien language of legacy code."
* **Interstellar Communication:** "May the docs be with you!"

**Additional Notes:**

* The "Interstellar Communication" commit type is best suited for commits that improve communication, collaboration, or
  documentation within a project.
* It emphasizes the importance of clear and effective communication in a complex system, like the vastness of space.
* Consider using this type for commits that add or update documentation, improve code comments, implement communication
  tools, or establish project guidelines.