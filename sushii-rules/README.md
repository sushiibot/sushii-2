# sushii-rules

Rules engine for processing Discord events with user configurable logic. In very
early development and mostly experimentation. This readme is mostly a design
doc and ideas, not actual documentation.

## Rule Configuration

1. Trigger
   * Discord gateway events
2. Conditions (Different condition types per event)
   * Messages
     * message content -> String conditions
     * message created -> DateTime conditions
     * channel_id -> Channel + Integer conditions
     * etc.
   * Member join
     * username -> String conditions
     * isBot -> bool conditions
     * isVerifiedBot -> bool conditions
     * previousNumberOfJoins -> Integer conditions
   * Counter
     * x times in last y seconds/minutes
     * rate is x in y minutes
   * Random
     * rand() = > < x
   * etc.
3. Actions
   * Discord actions
     * Message
     * Message via webhook
     * Add role
     * Ban
     * Kick
     * etc
   * sushii actions
     * Warn
     * Mute
   * Other actions
     * Sleep (sleeps longer than x minutes store in db and poll?)
     * trigger another rule? (prevent infinite looping, disallow recursive rules or add a TTL)
     * add/sub counter
     * Random action
       * sub actions*
     * Loop over multiple inputs
       * sub actions to loop
     * Save data
       * global (admin only)
       * guild
       * channel
       * member

## Rule Persistence

guild_rule_groups

* set of multiple rules
* a certain "feature" might contain a set of multiple rules

| id   | guild_id | name | description | enabled | editable | author | category | config |
| ---- | -------- | ---- | ----------- | ------- | -------- | ------ | -------- | ------ |
| uuid | bigint   | text | text        | bool    | bool     | bigint | text?    | jsonb  |

guild_rules

* rules can only have 1 trigger

| id   | rule_group_id           | rule_name | enabled | trigger | condition | actions |
| ---- | ----------------------- | --------- | ------- | ------- | --------- | ------- |
| uuid | fk guild_rule_groups.id | text      | bool    | text    | jsonb     | jsonb   |

### RuleStore Trait

Trait to easier handle different backend stores, basic starter MVP can use basic
json stores.

* `get_guild_rule`
* `save_guild_rule`

## Caching

On first trigger, rule is queried from db and then kept in memory for additional
calls. Can maybe use an LRU cache if it grows too large.

## Conditions

Conditions (boolean statements, e.g. x contains y) are grouped by data types to
make it easier for reuse and organization.

* Primitives-ish
  * String
    * ==
    * startsWith
    * contains word from word list
    * languageIs/IsNot/IsIn/IsNotIn (language-api)
    * % uppercase
    * % non-alphanumeric letters
    * number of lines
    * length
    * etc
  * Integers
  * DateTime
  * bool
* Discord Types - should be just using the underlying ID / integer comparisons,
  but UI should show separately + a list of guild channels)
  * Channel
  * Role
* sushii types
  * Warns
    * number of warns
  * Mutes
  * Quotas (rate limiting with governor crate)
    * number in last x minutes
    * number in a row
  * Level

## Actions

Actions should be able to reference to temporary data from conditions and other
actions, similar to github actions' `outputs`. This data should be stored in a
`RuleContext` which is *newly created* on each time a rule is triggered.

Main problem right now is assigning unique IDs to each condition/action, if this
should be done. This may cause it to be more confusing for end users. This is
mainly useful for multiple of the same condition / action and this will either
overwrite the previous one if they aren't unique, or will require users to
specify which one they want to use.

This context data is also passed to any handlebars text templating.

Example structure in json form.

```jsonc
{
  // Trigger data, e.g. a message
  "trigger": {
    "id": 12345,
    "content": "ping!",
    "author": {
      "name": "bob",
      "discriminator": 1234,
    }
  },
  // Condition data, should contain information on what data passed conditions
  // and the inputs / outputs
  "conditions": {
    "message": {
      "content": {
        "value": "!ping",
        "passed": true,
      }
    },
    "message.author.id": {
      "value": 123978123,
      "passed": true,
    }
  },
  // Actions
  "actions": {
    "send_message": {
      "id": 1234567890,
      "content": "pong!"
    }
  }
}
```

## Config

Rule sets can have a configuration with key value stores for different above
types. Rules can specify config keys to compare against. Could have a flag for
`repeatable` which allows an array of the same config keys in order to run the
same ruleset with different config values. (ie. reaction roles, same ruleset but
multiple configs for each role)

## Word List

For logical separation and easier data store for large lists, lists of words are
stored separately and conditions just reference a word list ID / name. These are
mostly just used for searches like x contains a word from word list (e.g.
for blacklisted words).  Larget lists should be searched with `aho-corasik` for
linear executing large number of word searches in linear time. These Aho–Corasick
FSM's should also be cached.

## Data Store

Actions can save data to be referenced for later.

| guild_id | target_type                                  | target_id | data |
| -------- | -------------------------------------------- | --------- | ---- |
| bigint   | enum {guild, channel, message, member, user} | bigint    | json |

User store is for admin only, for global features like rep and fishies.

## Interop with sushii-2

End goal is to replace most if not all features in sushii-2 with the rules
engine (excluding sushii special types and actions).

* sushii-2 should run alongside the rules engine process (if separate)
* database should be shared
* actions such as bans and mod logs should work as usual
  * creating pending entry in rules process and ban should still create mod log
    entry in main sushii-2 process
* HTTP API proxy should be required to not exceed rate limits

## TODO

* [x] Integrate proxy with `serenity-rs`, requires modification to serenity
* [ ] Serenity bot should serialize events into a redis queue to pass to rules
      process (a bit of redundant deserializing/serializing but CPU load isn't
      really a problem for now)
* [ ] Rules process should be able to read events from redis message queue and
      respond with Discord HTTP API requests via twilight + twilight http proxy
* [ ] Reimplementing features in rules engine means native features can be
      disabled in the main bot.
