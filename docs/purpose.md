# Contents
- [Contents](#contents)
- [The Purpose of this Repository](#the-purpose-of-this-repository)
- [Typical Solution Architecture](#typical-solution-architecture)
- [The WASM Promise 🤞 *(see webassembly.org)*](#the-wasm-promise--see-webassemblyorg)
- [How does Rust fit into this? 🤔](#how-does-rust-fit-into-this-)
  - [Original Assumption 🧐](#original-assumption-)
  - [The Big Reality Check 🤜💥😫](#the-big-reality-check-)
  - [So What Happened To spudmash-hack-randomuser-rust?](#so-what-happened-to-spudmash-hack-randomuser-rust)
  - [What Do I Actually Get from wasm-pack?](#what-do-i-actually-get-from-wasm-pack)
- [Is It Worth the Hassle? 🧐](#is-it-worth-the-hassle-)
- [TODO: Things to investigate](#todo-things-to-investigate)


# The Purpose of this Repository 
![why](/docs/img/why.jpg)

The majority of the repositories in Spudmash Media serve as Quickstart templates for your next software build.

This particular repository intends to solves the Client Side issue of type-safety of objects produced by Business Logic code without piling on layers of frameworks or remote server abstractions to achieve this.

# Typical Solution Architecture
When structuring your client side application a typical multi-tier architecture pattern is:
```
+--------+                     +-------------------+
| client |----[integration]----| 3rd Party Web Api |
+--------+                     +-------------------+
```

Where:

- client: is some front-end Single Page Application (SPA) rendered in a Web Browser like React.js / Vue.js with JavaScript code relying on the Fetch API from the browser. (see [MDN web docs "Fetch API"](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API))

- integration: is usually back-end server side code on Node.js | C# Web API | Python Flask | GraphQL etc.. and orchestrates HTTP calls and integration with 3rd party. 

- 3rd Party Web API: is an external resource


So in reality it would look similar to:

```
+-------------------------+
| [WebBrowser]            |
|+-----+   +-------------+|    +--------------------+    +-------------------+
|| app |---|  Fetch API  |*----| Business Logic API |----| 3rd Party Web Api |
|+-----+   +-------------+|    +--------------------+    +-------------------+
+-------------------------+       
```



Obviously the boundary of the integration layer can be blurred if your only platform is WEB and you consider integration code can be part of the client side app (this can be done with Angular with the HttpClient module OR even with vanilla JavaScript with the Fetch API)

```
+------------------------------------------+
| [WebBrowser]                             |
|+----------------------+   +-------------+|    +-------------------+
|| app + buisness logic |---|  Fetch API  |*----| 3rd Party Web Api |
|+----------------------+   +-------------+|    +-------------------+
+------------------------------------------+       
```

The risks are:
- JavaScript is not Type safe (there are NPM packages such as prop-types to help...but seriously come on! 😑)
- If we use TypeScript / Angular we're relying on another JavaScript library that transforms and bundles the strong type code down to ES5. 🤦‍♂️

Then came along Web Assembly...

# The WASM Promise 🤞 *(see [webassembly.org](https://webassembly.org/))*

> WebAssembly (abbreviated Wasm) is a binary instruction format for a stack-based virtual machine. Wasm is designed as a portable target for compilation of high-level languages like C/C++/Rust, enabling deployment on the web for client and server applications.

**Sweet** 👍

> The Wasm stack machine is designed to be encoded in a size- and load-time-efficient binary format. WebAssembly aims to execute at native speed by taking advantage of common hardware capabilities available on a wide range of platforms.
> 

**Liking this already!!** 👍👍

> WebAssembly describes a memory-safe, sandboxed execution environment that may even be implemented inside existing JavaScript virtual machines. When embedded in the web, WebAssembly will enforce the same-origin and permissions security policies of the browser.

**OK SOLD!!! SIGN ME UP!!!** 👍👍👍


So with that in mind the architecture changes slightly:

```  
+-------------------------+
| [WebBrowser]            |
|+-----+   +-------------+|    +-------------------+
|| app |---|  [Sandbox]  |+----| 3rd Party Web Api |
|+-----+   | WASM Module ||    +-------------------+
|          +-------------+|
|                         |
+-------------------------+
```


# How does Rust fit into this? 🤔

According to the Rust brochure (see [rust-lang.org](https://www.rust-lang.org/)), the language allows you to build code for the following applications:
- [Command Line](https://www.rust-lang.org/what/cli)
- **[WebAssembly](https://www.rust-lang.org/what/wasm)**
- Networking
- Embedded


## Original Assumption 🧐
After reading through the Rust guides and including the command line library (see [Learn Rust](https://www.rust-lang.org/learn)), building a library + command line tool targeting https://randomuser.me/api endpoint was trivial.
Full mapping of all the JSON objects to Rust structs was achieved and command line performance is adequate. (see spudmash-hack-randomuser-rust for details)

E.g. of mapped Rust response struct
```
//-------------------------------------------------------
// Struct:      RandomuserResponse
// Description: wrapper payload
//-------------------------------------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct RandomuserResponse {
    pub results: Option<Vec<UserStruct>>,
    pub info: Option<InfoStruct>,
}
```

Structuring the Rust project
```
+spudmash-hack-randomuser-rust
    |
    +- randomuser_core (contains randomuser.me models, constants, enums, behaviours, 
    |                   implementation of builders for HttpClients such as isahc*, reqwest, surf)
    |
    +- randu_cli (command line tool that utilizes the randomuser_core)


*NOTE: Isahc performed the best
```

The intention above was to have a randomuser_wasm module that reused all of randomuser_core and there was a sense the code "could" auto-magically compile to WASM using the wasm-pack tool and achieve this:
```  
+-------------------------+
| [WebBrowser]            |
|+-----+   +-------------+|    +-------------------+
|| app |---|  [Sandbox]  |+----| 3rd Party Web Api |
|+-----+   | WASM Module ||    +-------------------+
|          +-------------+|
|                         |
+-------------------------+
```

## The Big Reality Check 🤜💥😫

SOCKETS!!!! 😑 

The randomuser_core used crates (Rust libraries) that performed HTTP requests which relied on sockets - and this won't play well in WASM world.

The typical errors received were:

```
96 | / pub struct SockAddr {
97 | |     storage: sockaddr_storage,
98 | |     len: socklen_t,
99 | | }
   | |_- similarly named struct `SockAddr` defined here
```

When reading through [Rust and WebAssembly Book](https://rustwasm.github.io/book/) & [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/) quickly discovered (which is blindly obvious), the WASM code runs in a sandbox on the Web Browsers and has direct access to the Web Browser's APIs (such as Fetch API).


So with that in mind, the architecture quickly changed to:

```  

+----------------------------------+
| [WebBrowser]                     |
|+-----+            +-------------+|    +-------------------+
|| app |            |  Fetch API  |+----| 3rd Party Web Api |
|+--+--+            +-------------+|    +-------------------+
|   |                      |       |
|   |   +-------------+    |       |
|   +---| [Sandbox]   |----+       |
|       | WASM Module |            |
|       +-------------+            |
+----------------------------------+       

```


Which obviously meant restructuring the Rust project to:
```
+spudmash-hack-randomuser-rust
    |
    +- randomuser_core (contains randomuser.me models, constants, enums, behaviours)
    |
    +- randomuser_std (implementation of builders for HttpClients such as isahc*, reqwest, surf)
    |
    + randomuser_wasm (creates NPM package, uses randomuser_core + wasm-bindgen for web-sys: fetch)
    |
    +- randu_cli (command line tool that utilizes the randomuser_std)


*NOTE: Isahc performed the best
```

## So What Happened To spudmash-hack-randomuser-rust? 
During the implementation of randomuser_wasm, it was found the complex structure of RandomuserResponse did not work correctly with wasm-bindgen and failed to transform the code correctly with wasm-pack.

Hence a seperate crate was created and named it RANDU-CORE (this repository) which only used the models found from randomuser_core. Additional changes were made:
- a new View Model SimpleUser struct was created for the WASM module
- the WASM contained public function called **get_user_wasm** which orchestrated the call to randomuser.me, mapped the RandomuserResponse struct to a SimpleUser array, then returned the result to the consumer.

(NOTE: Will eventually migrate this code back to spudmash-hack-randomuser-rust)



## What Do I Actually Get from wasm-pack?

After building RANDU-CORE, you're left with an NPM package that contains:
- randu_core.wasm file that is 205kb in size
- randu_core.js proxy library to integrate with the wasm file if using vanilla NodeJS or React/Nextjs/Vue SPA apps
- randu_core.*.ts TypeScript definitions and implementation if you want to strongly typed objects in your project.

Integrating this NPM package to our client side SPA app achieves our architecture

```  

+----------------------------------+
| [WebBrowser]                     |
|+-----+            +-------------+|    +-------------------+
|| app |            |  Fetch API  |+----| 3rd Party Web Api |
|+--+--+            +-------------+|    +-------------------+
|   |                      |       |
|   |   +-------------+    |       |
|   +---| [Sandbox]   |----+       |
|       | WASM Module |            |
|       +-------------+            |
+----------------------------------+       

``` 


# Is It Worth the Hassle? 🧐

For NPM library maintainers definitely, especially business logic / proxy code.

I envisage carving out large amounts of javascript data mapping code in future web apps and replacing it with Rust WASM to guarantee of code correctness and consistent performance.

Note this code will also work on server side on NodeJS and Deno with the TypeScript definitions so definitely see a plus to using this tech stack.

However to finish off there a time and place for everything:
- Prototyping: go with integrating with JavaScript to quicker delivery projects/ideas.
- Production Ready: consider swapping out business logic code or process intensive code with Rust WASM for realiability.


# TODO: Things to investigate
- CORS
- XSS
- Actual performance gains in memory consumption and total request/resposne time compared to traditional javascript boilerplate code.
