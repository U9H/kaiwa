# Kaiwa

## Data Structure
App {
    Users []
    Sites [
        Domains[]
        Pages [
            Locations<String>
            Comments [
            ]
        ]
    ]
}

App {
    Config
}

User {

}

Sites {
    user_id : int # who owns it
    name : string # friendly name
    # HAS MANY Pages
    # HAS MANY Hosts
}

Host {
    site_id : int
    name : string
}

Page {
    # HAS MANY COMMENTS
}

## Quick Start

Install [Docker](https://docs.docker.com/engine/installation/) & [Docker-Compose](https://docs.docker.com/compose/install/)

Then:

```bash 
# clone template into new directory
git clone https://github.com/ghotiphud/rust-web-starter.git my-new-project
cd my-new-project

# start it up
docker-compose up
```
