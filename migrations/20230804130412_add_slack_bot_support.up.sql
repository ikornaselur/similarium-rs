CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
IF NOT EXISTS slack_bots (
    id uuid DEFAULT uuid_generate_v4() NOT NULL,
    app_id varchar(32) NOT NULL,
    enterprise_id varchar(32),
    enterprise_name varchar(200),
    team_id varchar(32) NOT NULL,
    team_name varchar(200),
    bot_token varchar(200),
    bot_id varchar(32),
    bot_user_id varchar(32),
    bot_scopes varchar(1000),
    bot_refresh_token varchar(200),
    bot_token_expires_at timestamp with time zone,
    is_enterprise_install boolean NOT NULL,
    installed_at timestamp with time zone NOT NULL,

    PRIMARY KEY (id)
);
