-- Add migration script here
ALTER TABLE user_info ADD CONSTRAINT MoneyMustBePositive CHECK ( money >= 0.0 );