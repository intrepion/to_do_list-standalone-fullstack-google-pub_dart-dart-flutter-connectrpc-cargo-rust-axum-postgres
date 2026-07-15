-- Migration: 003_add_updated_at_trigger_to_users
-- Description: Add updated_at auto-update trigger to users table
-- Up

-- Trigger for updated_at on users (function created in 002)
DROP TRIGGER IF EXISTS update_users_updated_at ON users;
CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Down (for rollback)
-- DROP TRIGGER IF EXISTS update_users_updated_at ON users;
