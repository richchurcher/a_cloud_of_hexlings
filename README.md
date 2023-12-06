# A Cloud of Hexlings

[Bevy Game Jame #4](https://itch.io/jam/bevy-jam-4) entrant, 2023.

## Elevator pitch

You are a hexagon. You spawn many hexlings. These are your drones. You are helpless without them.
They are helpless without you. Together, you must survive the hostile geometric darkness in which
you find yourself.

## Inspiration

### Homeworld

![homeworld2](https://github.com/richchurcher/bevy_jam_4/assets/171905/8b0f4c63-6fc6-428f-b4b5-d6d3e579f7e1)

Love those fighters drawing paths through the void with their drive plumes.

### Factorio

![Construction_robot_entity](https://github.com/richchurcher/bevy_jam_4/assets/171905/7e3ae2b2-7cf2-4506-b3cc-1912d929729c)

Bots are an intrinsic part of the mid to late game. A cloud of bots surrounds you, travelling away
from you and toward you, completing tasks.

### Assassin's Creed: Origins

![aco_senu](https://github.com/richchurcher/bevy_jam_4/assets/171905/76ac7948-68aa-4a2a-aba2-796d54ceeb2c)

Bayek and Senu's symbiotic relationship adds a wonderful dimension. Importantly, she is his eyes and
ears, but can also engage his foes. She dispells the fog of war and acts as his advance scout.

### Starcraft 2

![carriers](https://github.com/richchurcher/bevy_jam_4/assets/171905/f52b1fe0-f48b-4563-9178-1f1d8a47257c)

Carriers are just fun to play. They take a long time to build up to full strength, but are
devastating to certain enemy types.

## Player experience

In a roguelite style, the player moves through "rooms" in an uncertain environment, obscured by fog
of war. They must depend on their growing relationship with their spawned hexlings, learning more
about them and their capabilities as they progress.

## Platform

Web (WASM), Linux, Windows, MacOS.

## Development software

- **Bevy Engine**: rendering/ECS/etc
- **Neovim**: code and debugging
- **Bitwig Studio**: music and effects
- TODO: credit crates

## Genre

Story-driven roguelike.

## Target audience

People who like an off-beat concept, and who appreciate story provided alongside their mayhem might
enjoy this game. Also, geometry.

## Concept

### Gameplay overview

Initially, the player spawns a single, tiny hexling. They discover they are entirely dependent on it
to survive. In turn, it must be "fed" raw materials. It follows the player, and can be sent forth at
a distance or recalled to "home base".

As the story develops, the player sees text updates without interrupting gameplay. The number of
hexlings will gradually increase, and they will discover the need to "feed" them in order to
progress. The hexlings may gain abilities such as shield or additional firepower.

Boss fights will present themselves (probably in the form of giant geometric constructs... the
dreaded Octagonal Orofice, the monstrous Metagon whose number of sides cannot be rationally
comprehended, and so on).

### Jam theme and how we interpret it

_That's a LOT of Entities!_

I interpret the theme as the player's dependence on a cloud of hexlings, ever-growing, against
swarms of enemies that threaten to engulf them. Nothing too complex or esoteric going on here. We'll
see if various CPUs hold up under the strain.

### Primary mechanics

#### Rooms and corridors

For some reason, the player is inside. "Inside where?" is a question we do not necessarily need to
answer. However, they're contained, and they cannot pass through walls or doors so collision
detection required.

#### Fog of war

The player cannot see far past their own nose. In order to progress, they need to rely on their
hexlings revealing the local environment. This will lead to a claustrophobic experience, and heavy
dependence on their symbiotic relationship with their hexling friends.

#### Charge and recall

The hexlings are in one of three states at all times: _orbit_, _charge_, and _recall_. The player
orders a charge (at which point we signal this state by e.g. flashing the player mesh), and can
immediately reverse the hexling's direction by recalling them. This may lead to some interesting
micro with quick changes between the states.

#### Movement

The player can move freely within the environment constraints. How? Trigonometry, of course.

#### Feeding

The hexlings must be fed to recover their strength. A hungry hexling will still function, but at a
degraded level. They will eventually expire if not fed.

Feeding consists of ordering them to charge close enough to a resource-rich cluster of "food". They
eat green triangles.

### Secondary mechanics

#### Traps

Either the player or their hexlings may have their movement restricted or their abilities curtailed
by fixed traps within the environment. An evolution of the hexlings' powers could include heightened
"detect traps" ability.

## Art

Entirely "programmer art". We may be able to add skins to meshes if there's time, but gameplay
definitely has to come first.

## Audio

### Music

We call this style the ad-hoc number 8 wire cobbled together genre. Recorded sounds on mobile phone,
pasted into tracks. Ideally, it would be nice to use devices like 6/4 time, six-tone chords, etc. to
reflect the nature of the hexagonal beast.

### Effects

There is an attempt to make sounds within the game musical. Use of harmonics, ringing tones,
percussive notes etc. as events occur. Consistency in charge and recall effects e.g. a player should
start to automatically listen for the recall sound which will assist them in managing the rather
unconventional control scheme.

Perhaps boss fights can adopt more of a discordant approach. Bosses could use metre consistent with
the number of sides. We could also lean into harmonious, melodious content when the hexlings are at
rest or feeding.

## UI/UX

Consistent location of text updates, large enough to be easily read but unobtrusive. Fades after a
short time. Triggered by game events.

### Controls

- **WASD**: player movement
- **Space**: switch between charge and recall states
- **Left Shift**: spawn hexling(s)
- **Escape**: pause, with menu option to quit

If there's time:
- **R/Q**: fine-tune the hexlings' direction of travel, perhaps in an arc?

## Timeline

### Setup

**Complete by:** now-ish

- [x] Game design document
- [x] Create template repo
- [x] Update to latest Bevy
- [x] Remove/edit/personalise
- [x] Add MIT/Apache2 dual licenses

### Deploy early

**Complete by:** end of day Sunday.

- [x] Bare-bones title screen
- [x] "Hit space to begin"
- [x] "Hit escape to exit"
- [x] Space leads to _playing_ gamestate with player drawn
- [x] Escape leads to _pause_ gamestate with menu
- [x] Second escape hit exits
- [x] Space in _pause_ state returns to _playing_
- [x] Resurrect simple CI
- [x] WASM builds and can be published to itch.io
- [x] Linux builds and runs locally
- [x] Windows builds and runs locally

### Simple player movement

**Complete by:** end of day Sunday

- [x] Player can move WASD
- [x] Player can animate X axis flip (for charge and recall states)
- [x] Charge state changes color

### Simple environment

**Complete by:** end of day Tuesday

- [x] Procedural generation of rooms according to set dimensions?
- [x] Draw a room with an exit
- [x] Player collides with walls but can pass through exit

### Simple hexling spawns

**Complete by:** end of day Wednesday

- [x] Holding down left shift prompts a simple spinning animation, and a hexling appears!
- [x] Hexling follows player
- [x] Multiple hexlings can be spawned
- [x] Hexlings collide with environment, and each other, but not with exit
- [x] Hexlings do not get close enough to player to collide i.e. orbiting
- [x] Hexlings move out of player's way (perhaps scatter like bowling pins?)
- [x] Hexling charge (recall should just work?)

### Simple audio effects

**Complete by:** end of day Wednesday

- [ ] Charge harmonic
- [ ] Recall harmonic
- [ ] Spawn hexling harmonic
- [ ] Hexling collide harmonic? i.e. with each other and possibly with environment
- [ ] Player collide harmonic (with environment)

### Deploy all

**Complete by:** end of day Wednesday

- [ ] WASM
- [ ] Linux
- [ ] Windows

### Spawn single enemy in initial room

**Complete by:** end of day Thursday (at latest)

- [ ] Enemy has shape and randomly orbits around a fixed point
- [ ] Enemy will attack on proximity (either player or hexlings)
- [ ] Enemy will seek to collide with player, which reduces player health
- [ ] Hexlings can damage enemy by colliding with it, which reduces their health also
- [ ] Hexlings health does not reduce past a fixed point, but they become degraded
- [ ] Enemy despawns once destroyed
- [ ] If player is destroyed, _game over_ state displays
- [ ] In _game over_, space restarts
- [ ] In _game over_, escape exits

### Very simple soundtrack

**Complete by:** end of day Friday

- [ ] Don't reach for magical, we need quick and dirty
- [ ] Recorded and attached to _playing_ state
- [ ] Stops in _pause_ state
- [ ] If it's simple, add key controls to change volume
- [ ] Add enemy sound effects (harmonic or harsher/discordant?)

### Fog of war

**Complete by:** end of day Saturday

- [ ] Player can't see beyond r radius
- [ ] Player can see radius around each hexling
- [ ] Fog... looks like fog? If there's time.

### In-game text

**Complete by:** end of day Saturday

- [ ] Text events can appear based on in-game triggers
- [ ] Text eventually fades
- [ ] Basic story development and triggers for first room

### First level design

**Complete by:** end of day Saturday

- [ ] Say, four rooms with boss at end
- [ ] Distribute enemies throughout
- [ ] Distribute food for hexlings
- [ ] Hexlings can feed to regain health
- [ ] Ensure hexling damage is function of health
- [ ] For the purposes of the theme, allow a _lot_ of hexlings? But also a lot of enemies, for
  balance.
- [ ] Story development and triggers for first level

### Deploy all

**Complete by:** Sunday morning

- [ ] WASM
- [ ] Linux
- [ ] MacOS
- [ ] Windows

### More music

**Complete by**: end of day Sunday

- [ ] If you've gotten this far (nice work!) allow yourself the luxury of working on another music
  track, or making the first one more polished.
- [ ] Consider a brief title track or theme
- [ ] Add or polish sound effects
- [ ] Consider altering music during conflict
- [ ] Consider a boss fight track

### Second level design

**Complete by:** end of day Monday

- [ ] If complications haven't arisen, add another level
- [ ] Reduce hexling and enemy numbers in first level to provide growth
- [ ] Create another boss fight
- [ ] Create a portal or transition to the new level
- [ ] Consider adding a success/win screen

### Deploy all

**Complete by:** Monday night, probably laaate

- [ ] WASM
- [ ] Linux
- [ ] MacOS
- [ ] Windows

### SUBMIT

**Complete by:** Monday night

- [ ] Update itch.io description
- [ ] Links to all builds
- [ ] Ensure documentation and crate credits are provided
- [ ] Final checks and play tests
