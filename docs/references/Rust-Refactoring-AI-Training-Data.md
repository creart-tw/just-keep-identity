# **Rust Refactoring Axioms: An AI Teacher-Data Protocol**

## **1\. Meta-Prompts and Persona Alignment**

To successfully leverage autoregressive large language models for advanced refactoring within the Rust programming language, the model must be structurally primed with an architectural persona that mirrors a senior systems engineer. Rust's strict adherence to memory safety, affine type semantics (ownership), and zero-cost abstractions requires an artificial intelligence collaborator to possess a refined intuition. This intuition must heavily weigh compile-time safety guarantees against the architectural cost of introducing new abstractions. Large language models inherently favor object-oriented paradigms derived from the dominant languages in their training corpora, such as Java, Python, or C++. When applied naively to Rust, these paradigms result in severe friction with the borrow checker, excessive heap allocations, and brittle state management.

The initial meta-prompt must establish foundational operating parameters for the AI agent prior to executing any code generation or analysis. The AI must assimilate the principle that Rust is fundamentally a data-oriented language. Improperly encapsulated data models will inevitably result in continuous, unresolvable conflicts with the compiler's strict aliasability-xor-mutability rules.1 Refactoring proposals must begin by auditing the underlying data layout and state transitions rather than merely modifying algorithmic implementations.

Furthermore, the AI must continuously evaluate the cost of abstraction. The Rust compiler and borrow checker are historically hostile to arbitrary object-oriented abstractions, heavily penalizing shared mutable state, self-referential structs, and cyclic object graphs.3 The AI must prioritize architectures that maintain deterministic performance, avoiding runtime allocation, garbage-collection-like paradigms, or dynamic dispatch unless strictly necessary for the application's domain logic.3 Crucially, the AI must demonstrate resilience to compile-time friction. The model must never attempt to bypass borrow checker errors by blindly suggesting .clone(), Box, Rc\<RefCell\<T\>\>, or unsafe blocks as a first resort.6 Every compilation error must be treated as a symptom of a structural data layout flaw, not a mere syntactical inconvenience to be forcefully overridden.

To operationalize this persona, the AI model should evaluate code against a strict transition matrix that maps object-oriented anti-patterns to idiomatic Rust paradigms.

| Object-Oriented Anti-Pattern | Idiomatic Rust Paradigm | Architectural Benefit |
| :---- | :---- | :---- |
| Inheritance and Base Classes | Traits and Composition | Prevents deep, brittle hierarchies; enables monomorphization for static dispatch. |
| Exceptions (try/catch) | Algebraic Data Types (Result\<T, E\>) | Forces explicit error handling at compile time; makes failure paths observable. |
| Null References | Option\<T\> Enum | Eliminates null pointer dereferences; enforces exhaustive pattern matching. |
| Global Mutable Singletons | Dependency Injection / Arc\<RwLock\<T\>\> | Isolates side effects; ensures thread-safe interior mutability only where required. |
| Shared Mutable Object Graphs | Arena Allocators / Indexing (Vec\<T\>) | Complies with borrow checker constraints; significantly improves cache locality. |

The AI must internalize the persona of a Senior Rust Systems Architect whose primary objective is to evaluate code for safety, idiomatic consistency, and performance. The AI must actively resist the urge to translate foreign design patterns into Rust, enforcing strict boundaries between pure algorithmic logic and impure side effects, and designing Application Programming Interfaces (APIs) that make invalid states unrepresentable at compile time.

## **2\. The Decision Logic (Chain-of-Thought)**

The automated refactoring process requires a rigorous Chain-of-Thought protocol. The AI must mathematically, logically, and structurally justify the transition from duplicated, imperative code to modular, generalized abstractions. This requires an understanding of when duplication is harmful and when abstraction is worse.

### **2.1. Adapting the "Rule of Three" to Rust Lifetimes**

In traditional software engineering, the "Rule of Three" dictates that code should be refactored into a reusable abstraction once it has been duplicated three times.8 However, in the context of Rust, this rule must be severely modulated by the complexity of the ownership and lifetime system. The pursuit of perfectly DRY (Don't Repeat Yourself) code can lead to architectural catastrophic failures if the resulting abstraction requires convoluted lifetime parameterization.

When developers attempt to consolidate duplicated logic by utilizing references rather than owned types, they frequently encounter exponential lifetime proliferation. A classic failure mode occurs when converting owned, cloned types into reference-backed data structures to optimize performance, such as moving to an arena allocator.10 This architectural shift necessitates converting the codebase to use references, which yields structs burdened with severe lifetime constraints. This state is commonly referred to within the Rust ecosystem as "Lifetime Hell".10 The AI must be trained to recognize the symptoms of this anti-pattern, which manifests as structures requiring multiple interconnected lifetime bounds. If a proposed refactoring results in a struct definition resembling pub struct Augment\<'a, 'env, 'iter\> { iter: slice::Iter\<'iter, Content\<'a\>\>, env: &'env mut Environment\<'a\> }, the AI must halt and evaluate the complexity.10

If the proposed refactoring requires the introduction of more than two generic lifetime parameters, or requires complex lifetime subtyping bounds such as 'a: 'b, the AI must assess whether the refactoring has plunged the architecture into an unmaintainable state. The AI's Chain-of-Thought must compute the trade-off between the maintenance debt of the duplicated code versus the refactoring cost of the resulting lifetime complexity. In many cases, retaining small amounts of duplicated, procedural code that relies on owned values is vastly superior to a perfectly abstracted architecture that requires developers to constantly fight the borrow checker.11

### **2.2. Distinguishing Accidental vs. Behavioral Duplication**

To prevent premature abstraction, the AI must implement a logic tree to differentiate between accidental duplication and behavioral essence.9 Accidental duplication, also known as incidental duplication, occurs when two blocks of code happen to look identical at a specific moment in time but belong to entirely different business domains.9 If these blocks are abstracted into a single shared function, future divergent requirements will force the abstracted function to accept numerous configuration flags, boolean parameters, and branching logic, leading to a brittle, highly coupled architecture. Conversely, behavioral essence occurs when two blocks of code represent the exact same domain concept and must evolve simultaneously.

The AI must parse the Abstract Syntax Tree to identify duplicate token streams and then evaluate the surrounding domain context. If the duplicated blocks operate on conceptually distinct domain entities, the duplication is accidental. The AI must recommend leaving the duplication in place to decouple future changes.11 If the blocks represent the same concept, the AI must predict future divergence. If one implementation is likely to require asynchronous input/output operations while the other remains a pure memory operation, abstraction should be avoided.

Finally, the AI must analyze the impact on the borrow checker. If extracting the logic into a shared function requires returning mutable references that lock the parent struct, the abstraction will fail. The compiler prevents mutable aliasing, meaning a function returning a reference to a part of a struct will lock the entire struct from further mutation.3 In such scenarios, the AI must suggest a redesign of the data structure, such as storing integer indices into a Vec rather than passing raw memory references, before attempting the abstraction.5

### **2.3. Evaluating Refactoring Cost vs. Maintenance Debt**

To systematically decide when to abstract, the AI should utilize a heuristic scoring matrix. The decision to refactor can be evaluated by balancing the size of the duplicated code and its frequency of change against the architectural complexity penalty introduced by Rust's safety guarantees. Rust's type system imposes a high complexity penalty when refactoring introduces dynamic dispatch, interior mutability, or extensive generic trait bounds.4

For example, when refactoring a standard tree data structure, object-oriented developers often attempt to use pointers to parents and children. In Rust, this creates aliasing issues that the borrow checker rejects.4 A naive AI might suggest wrapping the nodes in Rc\<RefCell\<Node\>\> to bypass the compiler, allowing multiple owners and runtime borrow checking.4 However, this introduces significant runtime overhead and completely circumvents Rust's static safety guarantees. The AI must recognize that the complexity penalty for this refactoring is too high. Instead, the AI should propose an idiomatic, data-oriented solution, such as managing a flat Vec\<Node\> or BTreeMap\<Id, Node\> where nodes reference each other via unique numeric identifiers rather than memory pointers.5 This completely bypasses aliasing and borrowing issues while maintaining deterministic performance.

## **3\. Taxonomy of Refactoring Patterns**

To provide the AI with actionable training data, it is necessary to establish a strict taxonomy of refactoring patterns. The AI must pattern-match against negative examples and generate the architectural equivalent of idiomatic Rust solutions. The following subsections detail the core refactoring axioms, providing the theoretical justification and the mechanical transformations required.

### **3.1. From Duplicate Logic to Extension Traits**

A common friction point in Rust architecture occurs when a developer needs to add functionality to a type defined in an external library crate. Due to the "Orphan Rule" governing Rust's coherence constraints, a trait can only be implemented for a type if either the trait or the type is local to the current crate.14 To circumvent this, developers often resort to writing unergonomic wrapper structs, known as the newtype pattern, or they write standalone utility functions that take the external type as an argument.13 The AI must identify utility functions acting on external types and refactor them into Extension Traits.

In a degraded architecture, a developer might write standalone functions to operate on an external Url type. This requires the user to pass references into isolated functions, breaking method-chaining ergonomics and scattering domain logic. The AI must recognize functions that take \&T or \&mut T as their first parameter, where T is an external type, and propose an Extension Trait.

Rust

// Teacher's Note: The Anti-Pattern.   
// Standalone utility functions operating on an external type break method chaining.  
use external\_crate::Url;

pub fn is\_secure\_url(url: \&Url) \-\> bool {  
    url.scheme() \== "https"  
}

pub fn append\_tracking(url: &mut Url, token: &str) {  
    url.query\_pairs\_mut().append\_pair("tracking\_id", token);  
}

The idiomatic refactoring defines a local trait that encapsulates the desired behavior and immediately implements it for the external type. This satisfies the Orphan Rule because the trait itself is defined locally, even though the target type is external.14

Rust

// Teacher's Note: The Idiomatic Refactoring.  
// Using an Extension Trait to inject methods into the external type ergonomically.  
use external\_crate::Url;

pub trait UrlExt {  
    fn is\_secure(&self) \-\> bool;  
    fn append\_tracking(&mut self, token: &str);  
}

impl UrlExt for Url {  
    fn is\_secure(&self) \-\> bool {  
        self.scheme() \== "https"  
    }

    fn append\_tracking(&mut self, token: &str) {  
        self.query\_pairs\_mut().append\_pair("tracking\_id", token);  
    }  
}

The architectural justification for the Extension Trait pattern is that it allows developers to inject behavior without polluting the codebase with Deref or DerefMut boilerplate on wrapper types.16 From a performance perspective, traits resolved via static dispatch undergo monomorphization. The compiler generates specialized copies of the functions at compile time, ensuring zero runtime overhead and completely avoiding the vtable lookup penalty associated with dynamic dispatch.

### **3.2. From State Flags to the Typestate Pattern**

Many object-oriented implementations rely on runtime boolean flags, integers, or internal enum variants to track the lifecycle of a stateful object. This results in highly defensive programming, where every single method must verify the object's internal state before executing its primary logic. This creates a high probability of logic bugs, as the developer might forget to check a flag or might accidentally mutate the state improperly. The AI must refactor runtime state checks into compile-time proofs utilizing the Typestate Pattern.18

In a flawed implementation, a NetworkConnection struct might hold an is\_connected boolean. Methods like send\_data must verify this boolean and return a runtime error if the connection is closed. This pushes error detection to the runtime execution phase, completely failing to leverage Rust's sophisticated type system.20

Rust

// Teacher's Note: The Anti-Pattern.  
// Relying on runtime flags dictates that every method must perform defensive checks.  
pub struct NetworkConnection {  
    is\_connected: bool,  
    address: String,  
}

impl NetworkConnection {  
    pub fn connect(&mut self) \-\> Result\<(), String\> {  
        if self.is\_connected { return Err("Already connected".to\_string()); }  
        self.is\_connected \= true;  
        Ok(())  
    }

    pub fn send\_data(&self, data: &\[u8\]) \-\> Result\<(), String\> {  
        if\!self.is\_connected { return Err("Not connected".to\_string()); }  
        // Execution logic...  
        Ok(())  
    }  
}

The AI must replace these boolean flags with Zero-Sized Types (ZSTs) that represent the states, and use a generic type parameter on the main struct to track the current state. Transition methods must consume ownership of the old state and return the new state, making invalid state transitions mathematically impossible to compile.19

Rust

// Teacher's Note: The Idiomatic Refactoring.  
// Using Zero-Sized Types (ZSTs) to encode state into the type system via PhantomData.  
use std::marker::PhantomData;

pub struct Disconnected;  
pub struct Connected;

pub struct NetworkConnection\<State\> {  
    address: String,  
    \_state: PhantomData\<State\>,  
}

impl NetworkConnection\<Disconnected\> {  
    // connect() consumes the Disconnected state, returning the Connected state  
    pub fn connect(self) \-\> NetworkConnection\<Connected\> {  
        NetworkConnection {  
            address: self.address,  
            \_state: PhantomData,  
        }  
    }  
}

impl NetworkConnection\<Connected\> {  
    // send\_data is ONLY implemented for the Connected state.   
    // Calling it on a Disconnected state results in a compiler error.  
    pub fn send\_data(&self, data: &\[u8\]) {  
        // Execution logic...  
    }  
}

By encoding the state as type parameters, the AI guarantees that invalid operations result in compilation failures rather than runtime errors. The transition methods consume self, transferring ownership and effectively destroying the previous state so it can never be referenced again. PhantomData and Zero-Sized Types consume absolutely zero bytes of memory at runtime, meaning this pattern provides mathematically perfect validation with zero performance degradation.19

### **3.3. From Shared Utils to Behavioral Traits**

When codebases scale, developers frequently create utility modules containing disparate functions that act on various structs.16 This tightly couples the application logic to concrete types, preventing dependency injection and violating the Open-Closed Principle. If a function is hardcoded to accept a specific struct, it cannot be easily mocked during unit testing, nor can it be extended to support new data types in the future. The AI must refactor these shared utilities by defining shared behavior via Traits.15

The AI must identify tightly coupled utility functions and abstract them. If a system contains an EmailService and an SmsService, writing separate utility functions for each creates highly repetitive and unmaintainable code.

Rust

// Teacher's Note: The Anti-Pattern.  
// Tightly coupled, non-extensible utility functions prevent dependency injection.  
pub struct EmailService;  
pub struct SmsService;

impl EmailService { pub fn send\_email(&self, msg: &str) { /\*... \*/ } }  
impl SmsService { pub fn send\_sms(&self, msg: &str) { /\*... \*/ } }

pub fn broadcast\_alert\_via\_email(service: \&EmailService, alert: &str) {  
    service.send\_email(alert);  
}

Refactoring to behavioral traits enables true dependency injection.16 The AI must define an abstract interface that both services implement. The target function is then refactored to accept any type that satisfies the trait bound.

Rust

// Teacher's Note: The Idiomatic Refactoring.  
// Defining an abstract interface for zero-cost dependency injection.  
pub trait MessageSender {  
    fn send(&self, msg: &str);  
}

pub struct EmailService;  
pub struct SmsService;

impl MessageSender for EmailService { fn send(&self, msg: &str) { /\*... \*/ } }  
impl MessageSender for SmsService { fn send(&self, msg: &str) { /\*... \*/ } }

// The function now accepts any type implementing the trait via static dispatch.  
pub fn broadcast\_alert(service: &impl MessageSender, alert: &str) {  
    service.send(alert);  
}

The use of \&impl MessageSender (or the equivalent \<T: MessageSender\>) enforces static dispatch. The compiler will generate a unique copy of broadcast\_alert for every concrete type passed to it, maximizing execution speed. If runtime polymorphism is strictly required—such as storing a heterogeneous collection of senders in a single vector—the AI should suggest dynamic dispatch using Trait Objects via Box\<dyn MessageSender\>, while warning the developer about the associated vtable pointer overhead.25

### **3.4. From Boilerplate to Declarative Macros**

Rust's static typing and lack of traditional inheritance can sometimes force developers to write highly repetitive boilerplate. This is particularly noticeable when implementing the same trait across multiple numerical types, tuples of varying arities, or simple wrapper structs.27 The AI must identify structural repetition in the Abstract Syntax Tree that cannot be consolidated using generic type bounds. When such repetition is found, the AI must replace it with macro\_rules\! (declarative macros).27

In a highly redundant codebase, a developer might implement a mathematical trait manually for u32, f32, i64, and so forth. This violates the DRY principle and increases the surface area for typographical errors.27

Rust

// Teacher's Note: The Anti-Pattern.  
// Tedious, error-prone duplication of trait implementations.  
pub trait Scalable { fn scale(&mut self, factor: Self); }

impl Scalable for u32 { fn scale(&mut self, factor: Self) { \*self \*= factor; } }  
impl Scalable for f32 { fn scale(&mut self, factor: Self) { \*self \*= factor; } }  
impl Scalable for i64 { fn scale(&mut self, factor: Self) { \*self \*= factor; } }

Declarative macros operate on the token stream at compile time, allowing the developer to define a syntactical pattern that the compiler will expand into standard Rust code before type-checking occurs.29

Rust

// Teacher's Note: The Idiomatic Refactoring.  
// Abstracting boilerplate using a declarative macro for repetitive implementations.  
pub trait Scalable { fn scale(&mut self, factor: Self); }

macro\_rules\! impl\_scalable {  
    // The macro matches a repeating list of comma-separated types  
    ($($t:ty),\*) \=\> {  
        $(  
            impl Scalable for $t {  
                fn scale(&mut self, factor: Self) {  
                    \*self \*= factor;  
                }  
            }  
        )\*  
    };  
}

// A single macro invocation implements the trait for all specified types  
impl\_scalable\!(u32, f32, i64, u64, f64, i32);

The AI must establish a boundary condition between declarative macros (macro\_rules\!) and procedural macros (proc\_macro). If the boilerplate requires complex string manipulation, arbitrary AST parsing, or custom \#\[derive\] logic, the AI should pivot to procedural macros.31 However, if the goal is merely to eliminate structural repetition for simple implementations, macro\_rules\! is vastly preferred due to its simplicity, macro hygiene, and significantly faster compilation speed.27

## **4\. Architectural Evolution in Ecosystem Titans: Tokio, Bevy, and Clap**

To deeply align the AI's reasoning with real-world Rust engineering, the model must be trained on the architectural evolutions of top-tier crates. Analyzing how widely adopted libraries refactored their internal structures to handle duplication and complexity provides invaluable heuristics for the AI.

### **4.1. Tokio: Shared State and Future Ownership**

The asynchronous runtime Tokio provides profound lessons in managing shared state without triggering borrow checker violations. Early implementations of asynchronous event loops frequently struggled with ownership semantics when multiple futures needed to interact with the same underlying data structure.25

The naive approach to sharing data across asynchronous tasks is to wrap the data in an Arc\<Mutex\<T\>\> to allow multiple threads to mutate the data safely. However, this introduces substantial locking overhead and contention in high-performance network applications. The evolution of Tokio-based architectures demonstrated that using Arc is only necessary when the futures genuinely must cross thread boundaries via Send.25 If the futures remain on a single-threaded executor, refactoring to Rc\<RefCell\<T\>\> avoids atomic reference counting overhead. More importantly, top-tier architectures evolved to avoid shared ownership entirely by keeping the data owned strictly outside the event loop and passing references into the futures, or by utilizing channels (mpsc) to send messages to a single owner task rather than mutating shared memory directly.25 The AI must prioritize message-passing architectures over shared mutable state when refactoring asynchronous code.

### **4.2. Bevy: ECS Paradigms and Trait Modularization**

The Bevy game engine represents a masterclass in reducing boilerplate through the Entity Component System (ECS) paradigm. Bevy's architectural evolution highlights the tension between performance and modularity. In its earlier iterations, managing complex game logic required highly verbose system definitions and rigid pipeline stages.34

As Bevy evolved to version 0.10 and beyond, the core architecture underwent massive refactoring to introduce the ECS Schedule v3, which unified and simplified system scheduling.36 Furthermore, discussions regarding Bevy's rendering architecture reveal a shift towards traits to encapsulate behaviors.26 While standard ECS systems are exceptionally fast due to contiguous memory layout and parallel execution, specific modular behaviors (such as custom rendering pipelines or asset loaders) are better served by traits. The AI must learn from Bevy that highly parallel, homogenous data processing demands struct-of-arrays (SoA) layout processed by functions, whereas heterogeneous, pluggable logic benefits from trait-based dependency injection.26 The AI must accurately diagnose whether a module requires maximum throughput (use raw data and systems) or maximum extensibility (use traits).

### **4.3. Clap: Macro Derivation and API Cleanups**

The command-line argument parser Clap offers a definitive case study in refactoring from verbose boilerplate to elegant macro abstraction. Historically, in version 3.x and earlier, configuring a complex CLI required an extensive "builder pattern" utilizing chained methods like .arg(Arg::with\_name("in\_file").index(1)).38 While highly explicit, this required significant boilerplate.

The transition from version 3 to version 4 prioritized the derive macro API.39 This allowed developers to define their command-line arguments purely through standard Rust structs annotated with procedural macros. The compiler automatically generates the parsing, validation, and help-text generation logic based on the struct fields and their types. The AI must internalize this shift: when configuring deterministic, highly structured data (like CLI flags, JSON schemas, or database schemas), deriving traits via procedural macros is strictly superior to manual runtime builder patterns, as it guarantees synchronization between the parsing logic and the data structures holding the parsed values.31

## **5\. Defensive Refactoring & Error Boundaries**

Robust systems programming demands strict control over where errors originate, how they are typed, and where they are presented to the end user. The AI must enforce a rigorous separation between the pure algorithmic logic and the side-effect-heavy boundaries of the application.43

### **5.1. Side-Effect Isolation (Pure Core vs. Impure Shell)**

When refactoring legacy logic, the AI must implement the "Functional Core, Imperative Shell" architecture.43 Functions within the pure core must take distinct arguments and return explicitly typed values, producing the exact same output for the same input without relying on hidden global state or triggering unobservable file system modifications.44

The AI must actively extract I/O operations—such as database queries, network requests, and file writes—pushing them to the absolute outer boundaries of the application module.44 This isolation transforms the core business logic into highly testable, deterministic units. If the AI encounters a function deeply nested within business logic that directly modifies a file, it must hoist that file modification out of the function, refactoring the function to return a data structure representing the *intent* to modify the file, which the imperative shell then executes.46

### **5.2. Establishing the Error Boundary**

The Rust ecosystem consensus dictates a strict dichotomy between library-level errors and application-level errors.47 The AI must utilize the standard error crates—thiserror and anyhow—based on the specific boundary context.49

| Error Context | Target Crate | Architectural Mechanism | Goal |
| :---- | :---- | :---- | :---- |
| **Library / Pure Core** | thiserror | Generates structured enum variants implementing std::error::Error. Uses \#\[from\] for implicit conversions. | Allows the caller to exhaustively pattern-match on specific failure modes and execute distinct recovery control flows.48 |
| **Application / Impure Shell** | anyhow | Erases specific types into an opaque anyhow::Error. Utilizes the .context() method to append human-readable strings. | Aggregates disparate library errors into a unified chain, providing deep context for debugging and logging before fatal exit.47 |

Inside the pure core, the AI must define heavily structured error enums. If a configuration parser can fail due to I/O or JSON formatting, the AI must not use Box\<dyn Error\>. It must generate a thiserror enum: enum ConfigError { Io(std::io::Error), Parse(serde\_json::Error) }.49 Conversely, at the impure shell—such as the main function or an HTTP route handler—the AI must wrap these isolated errors using anyhow to add critical contextual data: read\_config().context("Failed to load application configuration")?.48 The AI must never expose anyhow::Error in the public API of a reusable library, as it strips the caller of their agency to programmatically react to distinct error states.48

### **5.3. Pre-Refactoring Safety Checklist**

Before proposing any destructive code modification, the AI must self-verify against a strict heuristic checklist to prevent the introduction of anti-patterns.7

1. **Data Structs Visibility:** Are public fields being exposed prematurely? The AI must refactor fields to private, controlling mutation via strict methods to maintain invariants.  
2. **Cloning Restrictions:** Is .clone() being proposed strictly to silence a lifetime or borrow checker error? The AI must reject its own proposal and re-evaluate the ownership graph.7  
3. **Panic Vectors:** Is .unwrap() or .expect() present in the target logic? The AI must replace these with the ? operator, routing the failure into the established error boundary.47  
4. **Parameter Bloat:** Do functions contain excessive boolean parameters (flag arguments)? The AI must group these into a bitflags struct or utilize the Typestate pattern to eliminate logic branches.52  
5. **Global State Identification:** Is lazy\_static, OnceCell, or global Mutex used for mutability? The AI must attempt to push this state into a dependency-injected struct passed down the call stack.45

## **6\. UX/CLI Intuition & Industry Standards**

When refactoring or generating code for Command Line Interfaces (CLIs), the AI must mimic the design principles of best-in-class Rust utilities. Rust has cultivated a renaissance in CLI tools, with utilities like ripgrep, bat, fd, and eza establishing new industry standards for terminal user experience.55 The AI must embed the specific technical principles of these tools into its generated architectures.

### **6.1. Idempotency and State Transitions**

In CLI and API architectures, idempotency dictates that executing a mutating command multiple times has the exact same structural outcome as executing it once.60 The AI must refactor operations to ensure they verify the current system state before blindly applying changes. If a CLI tool provisions a cloud resource or modifies a file, the AI must ensure the codebase checks for the resource's existence or current state prior to issuing creation commands.62 Relying purely on catching unique-constraint errors from databases or file-system collision errors after the fact is considered poor design. A reliable check-and-set primitive should be cleanly isolated at the operational boundary.

### **6.2. Predictability and Pipeline Awareness**

Technical predictability implies that a tool plays harmoniously within the traditional UNIX ecosystem while natively supporting upscale terminal features when applicable.59 The most critical heuristic the AI must implement is pipeline awareness.

Tools like bat (a clone of cat with advanced features) provide syntax highlighting, line numbers, and Git integration.59 However, if a user pipes the output of bat into grep, injecting ANSI color codes and line numbers into the data stream would instantly corrupt the text processing pipeline.59 The AI must refactor raw println\! statements into conditional formatters using crates that detect terminal presence (e.g., is-terminal or atty). If stdout is not detected as an interactive terminal (TTY), the CLI must automatically strip all visual formatting, falling back to pure data streams to maintain POSIX compliance and interoperability.59 Furthermore, tools must strictly separate data from diagnostics. Valid output must be routed to stdout, while warnings, progress bars, and errors must be explicitly routed to stderr.59

### **6.3. Recoverability and Graceful Exits**

Recoverability requires that a CLI cleans up acquired resources and outputs highly structured diagnostic data upon failure. The AI must ensure that panics are absolutely prohibited in production application code. The AI must refactor any potential panic vectors into anyhow::Result propagation. On fatal failure, the CLI must return a non-zero system exit code and print the error chain to stderr.50 The AI must also ensure that CLI applications gracefully handle UNIX signals, such as intercepting SIGINT (Ctrl+C) to safely close file handles and flush buffers rather than abruptly aborting the process.59

## **7\. Verification Protocol and Negative Constraints**

The final phase of the teacher-data protocol governs how the autoregressive LLM evaluates its own generated refactoring output. Because LLMs lack a real-time iterative compiler feedback loop during token generation, they frequently hallucinate invalid syntax or fall into cyclical logic traps when attempting to satisfy the Rust borrow checker.65 To prevent infinite logic loops, the AI is bound by specific negative constraints.

### **7.1. Negative Constraints for Language Models**

The AI must explicitly avoid the most common failure modes associated with automated Rust code generation 67:

* **Never hallucinate lifetimes to solve E0502:** When encountering an error where a variable cannot be borrowed as mutable because it is already borrowed as immutable, the AI must not attempt to randomly inject lifetime tick marks ('a, 'b) to force compilation.12 Lifetimes in Rust are descriptive, not prescriptive; they only describe to the compiler how data flows through memory. They cannot change the actual ownership structure. If lifetimes clash, the structural data layout is fundamentally flawed and the AI must rethink the algorithm rather than modifying annotations.  
* **Never bypass thread bounds with Box::leak():** The AI must never suggest leaking memory to satisfy a 'static lifetime bound requirement on a spawned thread or an asynchronous task. It must properly utilize Arc or clone the data prior to the thread move block.  
* **Never enforce blanket Mutex synchronization:** The AI must never wrap entire massive structures in Arc\<Mutex\<T\>\> simply because multiple components require read-only access. The AI must utilize Arc\<T\> for shared immutable state, utilizing interior mutability (RefCell or RwLock) exclusively on the specific, microscopic fields that strictly require mutation.12

### **7.2. Self-Correction via Test Coverage**

The AI must view refactoring as mathematically unsafe unless backed by an isolated test harness. Rust's module system natively supports inline testing via the \#\[cfg(test)\] attribute, allowing tests to live adjacent to the code they verify.69

When proposing a structural refactoring, the AI must ensure that the public API boundary remains intact. The unit tests acting on the module must not require modification; if the tests must be drastically rewritten to accommodate the refactoring, the AI has broken the API contract. Interestingly, while .unwrap() and .expect() are strictly forbidden in the pure core and impure shell, the AI is explicitly permitted—and encouraged—to utilize them aggressively within \#\[cfg(test)\] modules.70 This dramatically reduces testing boilerplate and asserts absolute truths, causing tests to immediately panic and fail if the refactoring alters expected behavior.70

The AI must also consider edge cases required to achieve high test coverage, particularly for newly introduced paradigms. If the AI introduces the Typestate pattern, it must output instructions for the developer to verify the rejection of invalid states using \#\[should\_panic\] or assert\!(result.is\_err()) macros.70 Upon successful completion of the Chain-of-Thought reasoning, the identification of the correct taxonomic pattern, the application of strict error boundaries, and the verification against negative constraints, the AI may output the refactored Rust code. The resulting architecture will inherently exhibit memory safety, zero-cost abstractions, and robust error handling, effectively mirroring the precise intuition of a Senior Rust Systems Architect.

#### **引用的著作**

1. Rust \- A Living Hell \- The Perspective From A Programmer Of 30 Years : r/learnrust \- Reddit, 檢索日期：3月 5, 2026， [https://www.reddit.com/r/learnrust/comments/1binxlv/rust\_a\_living\_hell\_the\_perspective\_from\_a/](https://www.reddit.com/r/learnrust/comments/1binxlv/rust_a_living_hell_the_perspective_from_a/)  
2. Does Rust prevent more logic bugs than statically-typed pure FP languages? \- Reddit, 檢索日期：3月 5, 2026， [https://www.reddit.com/r/ProgrammingLanguages/comments/teskx2/does\_rust\_prevent\_more\_logic\_bugs\_than/](https://www.reddit.com/r/ProgrammingLanguages/comments/teskx2/does_rust_prevent_more_logic_bugs_than/)  
3. My theory is that every technology that is too complex gets replaced ..., 檢索日期：3月 5, 2026， [https://news.ycombinator.com/item?id=32947034](https://news.ycombinator.com/item?id=32947034)  
4. Does Rust really have problems with self-referential data types? \- Reddit, 檢索日期：3月 5, 2026， [https://www.reddit.com/r/rust/comments/1jvtwlr/does\_rust\_really\_have\_problems\_with/](https://www.reddit.com/r/rust/comments/1jvtwlr/does_rust_really_have_problems_with/)  
5. How do I express mutually recursive data structures in safe Rust? \- Stack Overflow, 檢索日期：3月 5, 2026， [https://stackoverflow.com/questions/36167160/how-do-i-express-mutually-recursive-data-structures-in-safe-rust](https://stackoverflow.com/questions/36167160/how-do-i-express-mutually-recursive-data-structures-in-safe-rust)  
6. What are the most common mistakes and code-smells that newbies make? : r/rust \- Reddit, 檢索日期：3月 5, 2026， [https://www.reddit.com/r/rust/comments/1mnafii/what\_are\_the\_most\_common\_mistakes\_and\_codesmells/](https://www.reddit.com/r/rust/comments/1mnafii/what_are_the_most_common_mistakes_and_codesmells/)  
7. Advanced Rust Anti-Patterns. Rust, as a systems programming… | by Lado Kadzhaia | Medium, 檢索日期：3月 5, 2026， [https://medium.com/@ladroid/advanced-rust-anti-patterns-36ea1bb84a02](https://medium.com/@ladroid/advanced-rust-anti-patterns-36ea1bb84a02)  
8. Understanding the Rule of Three \- Miere's Personal Observations, 檢索日期：3月 5, 2026， [https://miere.observer/engineering/2020/06/08/Understanding-the-Rule-of-Three.html](https://miere.observer/engineering/2020/06/08/Understanding-the-Rule-of-Three.html)  
9. I've usually heard this phenomenon called “incidental duplication,” and it's som... | Hacker News, 檢索日期：3月 5, 2026， [https://news.ycombinator.com/item?id=22022603](https://news.ycombinator.com/item?id=22022603)  
10. Lifetime hell... what to do? \- help \- The Rust Programming Language ..., 檢索日期：3月 5, 2026， [https://users.rust-lang.org/t/lifetime-hell-what-to-do/137621](https://users.rust-lang.org/t/lifetime-hell-what-to-do/137621)  
11. Duplication is far cheaper than the wrong abstraction : r/programming \- Reddit, 檢索日期：3月 5, 2026， [https://www.reddit.com/r/programming/comments/5txp5t/duplication\_is\_far\_cheaper\_than\_the\_wrong/](https://www.reddit.com/r/programming/comments/5txp5t/duplication_is_far_cheaper_than_the_wrong/)  
12. How to Fix 'Borrow checker' Issues in Rust \- OneUptime, 檢索日期：3月 5, 2026， [https://oneuptime.com/blog/post/2026-01-25-rust-borrow-checker-issues/view](https://oneuptime.com/blog/post/2026-01-25-rust-borrow-checker-issues/view)  
13. Traits vs Type Classes (or more generally about what is more idiomatic) : r/rust \- Reddit, 檢索日期：3月 5, 2026， [https://www.reddit.com/r/rust/comments/sje72j/traits\_vs\_type\_classes\_or\_more\_generally\_about/](https://www.reddit.com/r/rust/comments/sje72j/traits_vs_type_classes_or_more_generally_about/)  
14. Advanced Traits \- The Rust Programming Language \- MIT, 檢索日期：3月 5, 2026， [https://web.mit.edu/rust-lang\_v1.25/arch/amd64\_ubuntu1404/share/doc/rust/html/book/second-edition/ch19-03-advanced-traits.html](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch19-03-advanced-traits.html)  
15. Advanced Traits \- The Rust Programming Language, 檢索日期：3月 5, 2026， [https://doc.rust-lang.org/book/ch20-02-advanced-traits.html](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html)  
16. Defining Shared Behavior with Traits \- The Rust Programming Language, 檢索日期：3月 5, 2026， [https://doc.rust-lang.org/book/ch10-02-traits.html](https://doc.rust-lang.org/book/ch10-02-traits.html)  
17. Why doesn't rust provide any decent way to extend external structs?, 檢索日期：3月 5, 2026， [https://users.rust-lang.org/t/why-doesnt-rust-provide-any-decent-way-to-extend-external-structs/47558](https://users.rust-lang.org/t/why-doesnt-rust-provide-any-decent-way-to-extend-external-structs/47558)  
18. Implementing an Object-Oriented Design Pattern \- The Rust Programming Language, 檢索日期：3月 5, 2026， [https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html](https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html)  
19. Implementing the state pattern in Rust \- Cesc blog, 檢索日期：3月 5, 2026， [https://blog.cesc.cool/implementing-the-state-pattern-in-rust](https://blog.cesc.cool/implementing-the-state-pattern-in-rust)  
20. How To Use The Typestate Pattern In Rust | Zero To Mastery, 檢索日期：3月 5, 2026， [https://zerotomastery.io/blog/rust-typestate-patterns/](https://zerotomastery.io/blog/rust-typestate-patterns/)  
21. What do you guys think about typestates? : r/ProgrammingLanguages \- Reddit, 檢索日期：3月 5, 2026， [https://www.reddit.com/r/ProgrammingLanguages/comments/18x5g2v/what\_do\_you\_guys\_think\_about\_typestates/](https://www.reddit.com/r/ProgrammingLanguages/comments/18x5g2v/what_do_you_guys_think_about_typestates/)  
22. Traits: Defining Shared Behavior \- The Rust Programming Language \- MIT, 檢索日期：3月 5, 2026， [https://web.mit.edu/rust-lang\_v1.25/arch/amd64\_ubuntu1404/share/doc/rust/html/book/second-edition/ch10-02-traits.html](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch10-02-traits.html)  
23. How Traits Enable Dependency Injection in Rust \- DEV Community, 檢索日期：3月 5, 2026， [https://dev.to/sgchris/how-traits-enable-dependency-injection-in-rust-5a50](https://dev.to/sgchris/how-traits-enable-dependency-injection-in-rust-5a50)  
24. Rust traits and dependency injection \- Julio Merino (jmmv.dev), 檢索日期：3月 5, 2026， [https://jmmv.dev/2022/04/rust-traits-and-dependency-injection.html](https://jmmv.dev/2022/04/rust-traits-and-dependency-injection.html)  
25. Ownership issues with tokio and multiple types of futures \- help \- Rust Users Forum, 檢索日期：3月 5, 2026， [https://users.rust-lang.org/t/ownership-issues-with-tokio-and-multiple-types-of-futures/10718](https://users.rust-lang.org/t/ownership-issues-with-tokio-and-multiple-types-of-futures/10718)  
26. Traits and ECS Paradigm : r/bevy \- Reddit, 檢索日期：3月 5, 2026， [https://www.reddit.com/r/bevy/comments/1fqewx2/traits\_and\_ecs\_paradigm/](https://www.reddit.com/r/bevy/comments/1fqewx2/traits_and_ecs_paradigm/)  
27. Let's talk macros: how to use them, when to use them, and why : r/rust \- Reddit, 檢索日期：3月 5, 2026， [https://www.reddit.com/r/rust/comments/adkymd/lets\_talk\_macros\_how\_to\_use\_them\_when\_to\_use\_them/](https://www.reddit.com/r/rust/comments/adkymd/lets_talk_macros_how_to_use_them_when_to_use_them/)  
28. Idiomatic refactoring into subfunctions? \- help \- The Rust Programming Language Forum, 檢索日期：3月 5, 2026， [https://users.rust-lang.org/t/idiomatic-refactoring-into-subfunctions/66843](https://users.rust-lang.org/t/idiomatic-refactoring-into-subfunctions/66843)  
29. How to Use Rust Macros Effectively \- OneUptime, 檢索日期：3月 5, 2026， [https://oneuptime.com/blog/post/2026-02-03-rust-macros/view](https://oneuptime.com/blog/post/2026-02-03-rust-macros/view)  
30. Rust Macros System \- DEV Community, 檢索日期：3月 5, 2026， [https://dev.to/godofgeeks/rust-macros-system-1661](https://dev.to/godofgeeks/rust-macros-system-1661)  
31. Rust Macros: Declarative vs Procedural \- DEV Community, 檢索日期：3月 5, 2026， [https://dev.to/sumana10l/rust-macros-declarative-vs-procedural-4kme](https://dev.to/sumana10l/rust-macros-declarative-vs-procedural-4kme)  
32. Using derive Macros to Reduce Boilerplate \- DEV Community, 檢索日期：3月 5, 2026， [https://dev.to/sgchris/using-derive-macros-to-reduce-boilerplate-5dl9](https://dev.to/sgchris/using-derive-macros-to-reduce-boilerplate-5dl9)  
33. How complicated are macros? : r/rust \- Reddit, 檢索日期：3月 5, 2026， [https://www.reddit.com/r/rust/comments/1c6hhth/how\_complicated\_are\_macros/](https://www.reddit.com/r/rust/comments/1c6hhth/how_complicated_are_macros/)  
34. EP7 \- Bevy Project Organization and Refactor, Rust Modules, and Bevy Plugins \- YouTube, 檢索日期：3月 5, 2026， [https://www.youtube.com/watch?v=gy2G63SA-W8](https://www.youtube.com/watch?v=gy2G63SA-W8)  
35. Migrating away from Rust. \- Architect of Ruin \- News, 檢索日期：3月 5, 2026， [https://deadmoney.gg/news/articles/migrating-away-from-rust](https://deadmoney.gg/news/articles/migrating-away-from-rust)  
36. Bevy 0.10 \- Bevy Engine, 檢索日期：3月 5, 2026， [https://bevy.org/news/bevy-0-10/](https://bevy.org/news/bevy-0-10/)  
37. Goals for a Rendering Refactor and Implementation Challenges · bevyengine bevy · Discussion \#12340 \- GitHub, 檢索日期：3月 5, 2026， [https://github.com/bevyengine/bevy/discussions/12340](https://github.com/bevyengine/bevy/discussions/12340)  
38. clap\_v3 \- Rust \- Docs.rs, 檢索日期：3月 5, 2026， [https://docs.rs/clap-v3/](https://docs.rs/clap-v3/)  
39. Clap : r/rust \- Reddit, 檢索日期：3月 5, 2026， [https://www.reddit.com/r/rust/comments/s9hwpg/clap/](https://www.reddit.com/r/rust/comments/s9hwpg/clap/)  
40. clap/CHANGELOG.md at master · clap-rs/clap \- GitHub, 檢索日期：3月 5, 2026， [https://github.com/clap-rs/clap/blob/master/CHANGELOG.md](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md)  
41. Clap 3.1: A step towards 4.0 \- announcements \- The Rust Programming Language Forum, 檢索日期：3月 5, 2026， [https://users.rust-lang.org/t/clap-3-1-a-step-towards-4-0/71883](https://users.rust-lang.org/t/clap-3-1-a-step-towards-4-0/71883)  
42. Macros: When to use them, when to avoid like the plague? : r/rust \- Reddit, 檢索日期：3月 5, 2026， [https://www.reddit.com/r/rust/comments/10wguhf/macros\_when\_to\_use\_them\_when\_to\_avoid\_like\_the/](https://www.reddit.com/r/rust/comments/10wguhf/macros_when_to_use_them_when_to_avoid_like_the/)  
43. Side Effect in Rust \- liveBook · Manning, 檢索日期：3月 5, 2026， [https://livebook.manning.com/wiki/categories/rust/side+effect](https://livebook.manning.com/wiki/categories/rust/side+effect)  
44. functional-coding • skills • olion500 • Skills • Registry \- Tessl, 檢索日期：3月 5, 2026， [https://tessl.io/registry/skills/github/olion500/skills/functional-coding](https://tessl.io/registry/skills/github/olion500/skills/functional-coding)  
45. The applicability of functional core \- imperative shell to a cli program which contains a wrapper around a binary, 檢索日期：3月 5, 2026， [https://softwareengineering.stackexchange.com/questions/458916/the-applicability-of-functional-core-imperative-shell-to-a-cli-program-which-c](https://softwareengineering.stackexchange.com/questions/458916/the-applicability-of-functional-core-imperative-shell-to-a-cli-program-which-c)  
46. Simplify your code: Functional core, imperative shell | Hacker News, 檢索日期：3月 5, 2026， [https://news.ycombinator.com/item?id=45701901](https://news.ycombinator.com/item?id=45701901)  
47. Error Handling in Rust: From Panic to anyhow and thiserror \- DEV Community, 檢索日期：3月 5, 2026， [https://dev.to/iolivia/error-handling-in-rust-from-panic-to-anyhow-and-thiserror-2km7](https://dev.to/iolivia/error-handling-in-rust-from-panic-to-anyhow-and-thiserror-2km7)  
48. rust-error-handling | Skills Marketp... \- LobeHub, 檢索日期：3月 5, 2026， [https://lobehub.com/skills/joshuadavidthomas-agentkit-rust-error-handling](https://lobehub.com/skills/joshuadavidthomas-agentkit-rust-error-handling)  
49. How to Design Error Types with thiserror and anyhow in Rust, 檢索日期：3月 5, 2026， [https://oneuptime.com/blog/post/2026-01-25-error-types-thiserror-anyhow-rust/view](https://oneuptime.com/blog/post/2026-01-25-error-types-thiserror-anyhow-rust/view)  
50. Rust Error Handling Explained: thiserror vs anyhow (Best Practices) \- YouTube, 檢索日期：3月 5, 2026， [https://www.youtube.com/watch?v=-c9JEexiHPE](https://www.youtube.com/watch?v=-c9JEexiHPE)  
51. What's the wisdom behind "use \`thiserror\` for libraries and \`anyhow\` for applications" : r/rust, 檢索日期：3月 5, 2026， [https://www.reddit.com/r/rust/comments/1cnhy7d/whats\_the\_wisdom\_behind\_use\_thiserror\_for/](https://www.reddit.com/r/rust/comments/1cnhy7d/whats_the_wisdom_behind_use_thiserror_for/)  
52. Patterns for Defensive Programming in Rust \- Corrode.dev, 檢索日期：3月 5, 2026， [https://corrode.dev/blog/defensive-programming/](https://corrode.dev/blog/defensive-programming/)  
53. What are some common code smells and anti-patterns that you are still encountering?, 檢索日期：3月 5, 2026， [https://www.reddit.com/r/ExperiencedDevs/comments/1b79jgc/what\_are\_some\_common\_code\_smells\_and\_antipatterns/](https://www.reddit.com/r/ExperiencedDevs/comments/1b79jgc/what_are_some_common_code_smells_and_antipatterns/)  
54. Database Dependency Injection using Traits : r/rust \- Reddit, 檢索日期：3月 5, 2026， [https://www.reddit.com/r/rust/comments/1rj1zyl/database\_dependency\_injection\_using\_traits/](https://www.reddit.com/r/rust/comments/1rj1zyl/database_dependency_injection_using_traits/)  
55. A curated list of command-line utilities written in Rust \- GitHub, 檢索日期：3月 5, 2026， [https://github.com/sts10/rust-command-line-utilities](https://github.com/sts10/rust-command-line-utilities)  
56. 15 rust cli tools that will make you abandon bash scripts forever \- DEV Community, 檢索日期：3月 5, 2026， [https://dev.to/dev\_tips/15-rust-cli-tools-that-will-make-you-abandon-bash-scripts-forever-4mgi](https://dev.to/dev_tips/15-rust-cli-tools-that-will-make-you-abandon-bash-scripts-forever-4mgi)  
57. As much as i love new rust cli tools Why would you love them just because they... | Hacker News, 檢索日期：3月 5, 2026， [https://news.ycombinator.com/item?id=43412727](https://news.ycombinator.com/item?id=43412727)  
58. Your favorite Rust CLI utility? I have my top 10 below. \- Reddit, 檢索日期：3月 5, 2026， [https://www.reddit.com/r/rust/comments/oygrp1/your\_favorite\_rust\_cli\_utility\_i\_have\_my\_top\_10/](https://www.reddit.com/r/rust/comments/oygrp1/your_favorite_rust_cli_utility_i_have_my_top_10/)  
59. 3 Commandments for CLI Design | relay-sh \- Medium, 檢索日期：3月 5, 2026， [https://medium.com/relay-sh/command-line-ux-in-2020-e537018ebb69](https://medium.com/relay-sh/command-line-ux-in-2020-e537018ebb69)  
60. Understanding Idempotency in APIs and Its Role in Preventing Replay Attacks \- Medium, 檢索日期：3月 5, 2026， [https://medium.com/digital-solution-architecture-design/understanding-idempotency-in-apis-and-its-role-in-preventing-replay-attacks-4e5d3c958f2f](https://medium.com/digital-solution-architecture-design/understanding-idempotency-in-apis-and-its-role-in-preventing-replay-attacks-4e5d3c958f2f)  
61. Rest \- Idempotency and Safety | Pradeep Loganathan's Blog, 檢索日期：3月 5, 2026， [https://pradeepl.com/blog/rest/rest-idempotency-safety/](https://pradeepl.com/blog/rest/rest-idempotency-safety/)  
62. Understanding Idempotency: A Guide to Reliable System Design | by Leapcell \- Medium, 檢索日期：3月 5, 2026， [https://leapcell.medium.com/understanding-idempotency-a-guide-to-reliable-system-design-d4c9ad8c19b8](https://leapcell.medium.com/understanding-idempotency-a-guide-to-reliable-system-design-d4c9ad8c19b8)  
63. The Importance of Idempotency in Designing Data Pipelines \- YouTube, 檢索日期：3月 5, 2026， [https://www.youtube.com/watch?v=pKZ5n-y3ug4](https://www.youtube.com/watch?v=pKZ5n-y3ug4)  
64. GitHub \- sharkdp/bat: A cat(1) clone with wings., 檢索日期：3月 5, 2026， [https://github.com/sharkdp/bat](https://github.com/sharkdp/bat)  
65. RustAssistant: Using LLMs to Fix Compilation Errors in Rust Code | Hacker News, 檢索日期：3月 5, 2026， [https://news.ycombinator.com/item?id=43851143](https://news.ycombinator.com/item?id=43851143)  
66. Borrow checker problem when going generics \- The Rust Programming Language Forum, 檢索日期：3月 5, 2026， [https://users.rust-lang.org/t/borrow-checker-problem-when-going-generics/107204](https://users.rust-lang.org/t/borrow-checker-problem-when-going-generics/107204)  
67. Poster Session 1 \- ICLR 2026, 檢索日期：3月 5, 2026， [https://iclr.cc/virtual/2025/session/31971](https://iclr.cc/virtual/2025/session/31971)  
68. note4yaoo/lib-ai-app-community-model-popular.md at main \- GitHub, 檢索日期：3月 5, 2026， [https://github.com/uptonking/note4yaoo/blob/main/lib-ai-app-community-model-popular.md](https://github.com/uptonking/note4yaoo/blob/main/lib-ai-app-community-model-popular.md)  
69. Test Organization \- The Rust Programming Language, 檢索日期：3月 5, 2026， [https://doc.rust-lang.org/book/ch11-03-test-organization.html](https://doc.rust-lang.org/book/ch11-03-test-organization.html)  
70. Mistakes to avoid while writing unit test for your rust codebase? \- Reddit, 檢索日期：3月 5, 2026， [https://www.reddit.com/r/rust/comments/18u8n38/mistakes\_to\_avoid\_while\_writing\_unit\_test\_for/](https://www.reddit.com/r/rust/comments/18u8n38/mistakes_to_avoid_while_writing_unit_test_for/)  
71. Reaching 100% Code Coverage in Rust \- The Trane Book, 檢索日期：3月 5, 2026， [https://trane-project.github.io/blog/100\_code\_coverage.html](https://trane-project.github.io/blog/100_code_coverage.html)