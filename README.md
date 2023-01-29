# Minecraft Bedrock LAN Advertizer

**Keywords**: MCPE, Minecraft Bedrock, Friends tab, LAN games, cannot join local dedicated server.

**Abbreviations**

MCBE - **M**ine**c**raft **B**edrock **E**dition

MS - **M**icro**s**oft

# What can this program do

An MCBE client sends out special network broadcasts called Pings and asks all other devices on the network if they have a server running. 

Say you have a phone and an iPad. If you start a LAN world on an iPad and then open minecraft on the phone, your phone will ask the iPad if it has a server (Pings the iPad). The iPad will say _"Yes, on port 12345"_ (Thats a Pong). The phone will take iPad's IP address, that port 12345 and try to connect to it.

However a dedicated minecraft server that you run on a computer or laptop may not respond to your phone's Pings for whatever reason. Even if you are on the same network. So this program will do that for the server. This way if you cannot use the _Servers_ tab because of parental restrictions/don't have a microsoft account logged in then you can still join your dedicated LAN server that runs on your laptop.

This is also useful if the dedicated server does not accept 2 players under 1 MS account so you may want to disable online mode and maybe log out from your MS account on one of the devices.

# What this program cannot do

If the server's IP is not the same IP from which the Pong packet was sent then you will not be able to connect to the server. This is because the IP address is not part of the Pong packet. Instead the Pong sender's IP is the IP address that the client will connect.

So, no you cannot join some-server.net using this program. If really want to do that I can only point you to setting up a proxy. This is beyond the scope of this application.

Special thanks to [u/Riven5's suggestion](https://www.reddit.com/r/MCPE/comments/10945ek/comment/j3xn26b/?utm_source=share&utm_medium=web2x&context=3) to use [jhead/phantom](https://github.com/jhead/phantom). As Phantom's readme says:

> **Phantom README.md:**
> 
> Makes hosted Bedrock/MCPE servers show up as LAN servers, specifically for consoles.
> You can now play on remote servers (not Realms!) on your Xbox and PS4 with friends.
> It's like having a LAN server that's not actually there, spooky.

I am not related to phantom in any way so if you have any questions about that software please contact the developer in their prefered way. Their README seems to have enough instructions on how to use the software so you should not have any issues.

# Running

## Relay mode

**Advantages:**

- The information presented to the client will be up-to-date. Meaning the name, descrition, player count, etc. Everything will be the same as the server says.
- If the server is not running you will not see anything in the _Friends_ tab.

Running:

``` shell
advertize relay [ optional path to server.properties ]

# E.g:
  advertize relay bedrock_servers/creative_1/server.properties
# You have to give a path to the file itself, not just the directory,
# otherwise the file won't be found.
```

In this mode the program will look into the current directory for a file called 
`server.properties`. In this file it will look for the `server-port`. Then it will
start a UDP socket on port 19132 and when it recieves a Ping it will forward it to
`0.0.0.0:server-port`. And the responce from that (a Pong) will be sent back to the
client that Pinged the program. This is useful if you want to accurately reflect
the number of players online.

## From properties mode

**Disadvantages:**

- The app will not check if the server is actually running. So while you may see the button to join the server, it is not guaranteed that the server is actually running, or that the server is running on the port the client thinks it runs on. 

**Advantages**

- If the server ignores this app's Pings and you dont see anything in the _Friends_ tab, you can try this mode. There is a **caveat**: Make sure that the port in server.properties that the program uses is the same as the port for the server. You can remove the need to check by simply running the app in the same directory as your server or giving it a path the correct file.

``` shell
advertize from_props [ optional path to server.properties ] proto <num> <str>

# E.g:
  advertize from_props creative_world/server.properties proto 560 '1.19.51'
```

In this mode the `server.properties` is read and only the information there is used
to respond to Pings. In this mode online player count will always be 0. Look up `<num>` and `<str>` in [Bedrock protocol version numbers](https://wiki.vg/Bedrock_Protocol_version_numbers). `num` is the single number (560) and `str` is the 1.19.51 representation.

# Configuration

In the project's github you will find the sample `server.properties`. It only contains
the configuration that is required by this program, but it is not sufficient for an
MCBE server. Ideally you will run the `advertize` executable in the same directory as the
bedrock server executable.

# Installation

Currently we recommend to use `cargo install`. We might provide binary releases in the future.

## Using cargo

You need to have cargo and rust installed for this.

``` shell
cargo install mcbe-lan-advertizer
```

Then either:

- Navigate to the directory where your server is located.
- Create a `server.properties` in the current directory.
- Specify the path to `server.propeties`.

## Build from source

Run the following commands in your terminal (`cmd.exe` on Windows).

1. Clone the repository and enter it's directory. You have to have `git` installed for this.

``` shell
git clone https://github.com/max-ishere/mcbe-lan-advertizer
```

2. Build the application. You will need `cargo` and rust installed.

``` shell
cargo build --release

# --release will optimize the app so it runs more efficiently
```

3. Copy the executable somewhere convinient.

``` shell
cp target/release/advertize (path)
```
You can also do this in a file manager by opening the project, then `target` and then `release` folder. 

4. If you dont need the git repository and you have saved the executable to a different location then you can delete it.

``` shell
cd ..
rm mcbe-lan-advertizer
```

# Have questions?

Ask on [Discussions](https://github.com/max-ishere/mcbe-lan-advertizer/discussions).
