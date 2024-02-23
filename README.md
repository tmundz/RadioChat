# Radio Chat

# Purpose
I want to expand my software development skills specifically to better understand TCP/IP, HTTP, Sockets, webRTC, and Web Sockets. I aim to create a Broadcast type application written in Rust using react for the UI. Think of Twitch but only a live audio stream where users can listen in and send messages to the broadcaster.


## Development Process
### Essential Functionalities 
- [x] Set up Postgres db with docker
- [x] Create Users
- [x] Login Users
- [ ] set up JWT to track sessions
- [ ] set up Admin users
- [ ] Let users Create Channels
- [ ] Allow Broadcasters Delete Their Channels
- [ ] Let enter Listeners go into Channels
- [ ] Let users join Channels they do not own
- [ ] Differentiate Broadcasters and Listeners
- [ ] Set up WebRTC Connection
- [ ] Broadcast Live Audio to Listeners
- [ ] Set up Websocket Connection
- [ ] Allow Listeners send messages to Broadcaster

### Secondary Functionalities
- [ ] Allow Broadcasters create moderators
- [ ] Allow Broadcasters and Moderators ban and kick users

### Admin Functionalities
- [ ] Create pages specific for Admins

### CI
- [ ] Make sure user information updates properly
- [ ] Ensure Channels are Consistent



## Server Tools
- Rust
- Tokio
- Actix
- SQLx

- Docker
- Postgres

## UI Tools
- React (web ui)
- Leptos/Qt (desktop) if possible
- Flutter/Java (Android UI) if possible



