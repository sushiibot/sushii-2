INSERT INTO guild_configs
     VALUES ($1,  $2,  $3,  $4,  $5,  $6,  $7,  $8,  $9,  $10,
             $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
             $21, $22, $23, $24, $25, $26, $27)
ON CONFLICT (id)
    DO UPDATE 
        SET -- id = $1, Don't need to update ID 
            name              = $2,
            prefix            = $3,
            join_msg          = $4,
            join_msg_enabled  = $5,
            join_react        = $6,
            leave_msg         = $7,
            leave_msg_enabled = $8,
            msg_channel       = $9,
            role_channel      = $10,
            role_config       = $11,
            role_enabled      = $12,
            invite_guard      = $13,
            log_msg           = $14,
            log_msg_enabled   = $15,
            log_mod           = $16,
            log_mod_enabled   = $17,
            log_member        = $18,
            mute_role         = $19,
            mute_duration     = $20,
            ban_dm_text       = $21,
            ban_dm_enabled    = $22,
            kick_dm_text      = $23,
            kick_dm_enabled   = $24,
            mute_dm_text      = $25,
            mute_dm_enabled   = $26,
            max_mention       = $27
