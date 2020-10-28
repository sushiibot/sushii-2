settings_cmds!(leavemsg, leave_msg, leave_msg_enabled, {
    let leave_msg = args.rest();

    if leave_msg.is_empty() {
        msg.channel_id
            .say(
                &ctx.http,
                "Error: Give a leave message. You can use the placeholders \
                    `<mention>`, `<username>`, `<server>` to get the corresponding values.",
            )
            .await?;

        return Ok(());
    }

    leave_msg
});
