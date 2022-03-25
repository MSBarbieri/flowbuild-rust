-- This file should undo anything in `up.sql`
DROP INDEX idx_timer_expiration;
DROP TABLE timers;

DROP TABLE activities;
DROP INDEX idx_am_action;
DROP INDEX idx_am_started;
DROP TABLE activity_managers;

ALTER TABLE processes DROP COLUMN current_state_id;
DROP INDEX idx_process_state_process_id;
DROP TABLE process_states;
DROP TABLE processes;

DROP TABLE packages;
DROP INDEX idx_workflows_id;
DROP INDEX idx_workflows_name;
DROP TABLE workflows;
Drop SEQUENCE workflow_version_seq;