# CICD Pipeline 🔄

## Requirement

This pipeline is built with [GitHub Actions](https://help.github.com/en/actions) to perform:

- Build Rust code & validate unit tests
- Build & tests a NPM package that contains
  - WASM module
  - JavaScript library that integrates with the WASM module
  - TypeScript library that integrate with the WASM module
- Deploy the NPM package to GitHub Packages


## Anatomy of GitHub Actions Workflow file

```
name: <Name to appear in GitHub Actions>
on:
    push:
        branches: [<branch_name>]
  pull_request:
    branches: [<branch_name>]
env:
    <environment_variable>:<environment_value>
jobs
    <job_name>:
        runs-on: <OS_TYPE>

        steps:
            - name: <step_name>
              uses: <pre_made_workflow_in_-_GitHub> 

        ...OR...

        steps:
            - name: <step_name>
              run: <shell_command>

        ..OR...

        steps:
            - name: <step_name>
              run: |
              <multi_line_shell_command>

```

## This Workflow (rust_wasm.yaml)

A Rust Wasm workflow is not available so a custom file was created.

The file can be located in the repository at [.github/workflows/rust_wasm.yaml](../.github/workflows/rust_wasm.yaml)

It is split into 2 jobs
- wasm_pack
  - setup actions and tools (checkout code + install wasm-pack)
  - run cargo unit test
  - run wasm-pack integratioin tests
  - run wasm-pack build to produced an NPM package
  - upload the NPM to temporary storage for next build job + available to download as a zip file 
  
- npm_publish
    - setup Node environment
    - download built NPM package in previous build
    - publish the content to GitHub Packages

# Questions💡
## Q1. Why didn't I clump everything into a single job?
Definitely could do that, however the desire to swith the job into 2 is for the purpose of:
-  verify the contents by downloading the zip file if required
-  allowing rerunning of failed jobs without triggering the entire build process again. For example, if the jobnpm_publish failed for any reason, I can rerun the npm_publish step without triggering the entire build processagain (TODO: need to investigate why caching of jobs isn't working)

## Q2. GitHub Packages needs an Access Token, where do I get one?
By default, each repo has a **GITHUB_TOKEN** hidden token. Just need to reference it in your build step

```
    - name: "[NPM Publish to GitHub Packages (*note: navigate to package.json before running npm)]"
        run: cd wasm_output &&  npm publish --verbose
        env:
          NODE_AUTH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Q3. Whats the difference between the 2 GitHub Actions variables substitution types $NAME and ${{ NAME }}?

### 1) $THE_LINUX_STYLE_VARIABLE

Use this type of variable in **run** commands

E.g.
```
env:
  NPM_SCOPE: "spudmashmedia"

jobs:
    wasm_pack:
        steps:
            - name: "[wasm-pack build]"
              run: wasm-pack build --scope $NPM_SCOPE
```

### 2) ${{ THE__STYLE_VARIABLE }}

Use this type of variable in Workflow specific commands such as **with**

E.g.

```
env:
  NPM_PUBLISH_CONFIG_URL: "https://npm.pkg.github.com"
  NPM_SCOPE: "spudmashmedia"


jobs:
    npm_publish:
        runs-on: ubuntu-latest
        needs: wasm_pack
        strategy:
            matrix:
                node-version: [12.x] # no quotes on the node versions
        steps:
            - name: "[Setup Node ${{ matrix.node-version}}]"
              uses: actions/setup-node@v1.4.2
              with:
              node-version: ${{ matrix.node-version }}
              registry-url: "${{ env.NPM_PUBLISH_CONFIG_URL }}"
              scope: "@${{ env.NPM_SCOPE }}"
```
