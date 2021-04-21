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

| id   | rule_group_id           | rule_name | trigger_event | enabled |
| ---- | ----------------------- | --------- | ------------- | ------- |
| uuid | fk guild_rule_groups.id | text      | text          | bool    |

guild_rule_conditions

* json data of all conditions for given rule

| id   | rule_id           | condition data |
| ---- | ----------------- | -------------- |
| uuid | fk guild_rules.id | jsonb          |

guild_rule_actions

* json data of all action steps

| id   | condition_id                | actions data |
| ---- | --------------------------- | ------------ |
| uuid | fk guild_rule_conditions.id | jsonb        |

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

Actions should be able to reference to temporary data from other actions,
similar to github actions' `outputs`.

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
