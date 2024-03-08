use clap::Parser;
use serde::Serialize;

mod repo;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Name of the service, library, or other unit of code being built
    #[arg(long)]
    target: String,

    /// Name of the project that contains the target
    #[arg(long)]
    component: String,

    /// Version of the software being built
    #[arg(long)]
    component_version: String,
}

#[derive(Serialize)]
struct ManifestSource {
    status: String,
    //hash: String,
    //long_hash: String,
    //origin: String,
    //tags: Vec<String>,
    //branch: Vec<String>,
}

#[derive(Serialize)]
struct Manifest {
    target: String,
    component: String,
    version: String,
    source: repo::Manifest,
}

fn build_manifest_source() -> repo::Manifest {
    let repo = repo::Repo::new(None);
    repo.manifest()
}

fn build_manifest() -> Manifest {
    let args = Args::parse();
    let source: repo::Manifest  = build_manifest_source();
    Manifest {
        component: args.component,
        target: args.target,
        version: args.component_version,
        source: source,
    }
}

fn main() {
    let manifest: Manifest = build_manifest();
    let manifest_json: String = serde_json::to_string(&manifest).unwrap();
    println!("{}", manifest_json);
}

/*
  "buildTime": "{{ BUILD_TIME }}",
  "source": {
    {% if GITHUB_SHA -%}
      "id": "{{ GITHUB_SHA }}",
    {% else -%}
      "id": "{{ EARTHLY_GIT_HASH }}",
    {% endif -%}
    "origin": "{{ EARTHLY_GIT_ORIGIN_URL }}",
    {% if GITHUB_REF_NAME -%}
      "ref": "{{ GITHUB_REF_NAME }}",
    {% else -%}
      "ref": "{{ EARTHLY_GIT_BRANCH }}",
    {% endif -%}
    "status": "{{ GIT_STATUS }}"
    {%- if GITHUB_SERVER_URL and GITHUB_REPOSITORY -%},
    "url": "{{ GITHUB_SERVER_URL }}/{{ GITHUB_REPOSITORY }}"
    {%- endif %}
  },
  "ci": {
    "earthly": {{ EARTHLY_CI }},
    "github": {{ "true" if GITHUB_SHA else "false" }}
    {%- if GITHUB_RUN_NUMBER -%},
    "build-number": "{{ GITHUB_RUN_NUMBER }}"
    {%- endif %}
    {%- if GITHUB_RUN_ID -%},
    "run-id": "{{ GITHUB_RUN_ID }}"
    {%- endif %}
  }
  */


