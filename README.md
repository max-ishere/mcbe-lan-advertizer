# Minecraft Bedrock LAN Advertizer

MCBE - **M***ine**c**raft **B**edrock **E**dition

------------------

# What can this program do

An MCBE client sends out special network broadcasts called Pings and 
asks all other devices if they have a server running. If you say have a phone and
an iPad and you start a LAN world on an iPad and then open minecraft on the phone, 
your phone willask the iPad if it has a server (Ping). The iPad will say *"Yes, on 
port 12345"* (Thats a Pong). The phone will take iPad's IP address, that port 
12345 and try to connect to it.

However a dedicated minecraft server that you run on a computer/laptop may not respond
to your phone's Pings for whatever reason. So this program will do that for the server. 
This way if you cannot use the Servers tab because of parental restrictions/don't have 
a microsoft account logged in then you can still join your dedicated server that runs 
on your laptop.

# What this program cannot do

If you want to use "Friends" tab to join a remote server you found on the internet
you cannot do that! This is because the address of the computer that responds to the
Ping with a Pong will be the address that the client will connect to. So if you have
be-server.net, port 19132 and you run this program, it may show up as "Some bedrock 
server" but you will connect not to be-server.net but to your local computer, which
wont let you onto the server.

There is a way to connect to remote servers from your friends tab but for that to work
you will need a minecraft UDP relay. This is beyond the scope of this application.

# Modes of operation

## Relay

``` shell
advertize relay [ optional path to server.properties ]
```

In this mode the program will look into the current directory for a file called 
`server.properties`. In this file it will look for the `server-port`. Then it will
start a UDP socket on port 19132 and when it recieves a Ping it will forward it to
`0.0.0.0:server-port`. And the responce from that (a Pong) will be sent back to the
client that Pinged the program. This is useful if you want to accurately reflect
the number of players online but not very useful otherwise.

## From properties

``` shell
advertize from_props [ optional path to server.properties ]
```

In this mode the `server.properties` is read and only the information there is used
to respond to Pings. In this mode online player count will always be 0.

# Configuration

In the project's github you will find the sample `server.properties`. It only contains
the configuration that is required by this program, but it is not sufficient for an
MCBE server. Ideally you will put `advertize` executable in the same directory as the
bedrock server executable.
