CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- workflows
CREATE SEQUENCE workflow_version_seq;

CREATE TABLE workflows (
  "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  "name" VARCHAR(255) NOT NULL,
  "description" TEXT,
  "blueprint_spec" JSONB NOT NULL,
  "blueprint_hash" TEXT NOT NULL,
  "version" INT NOT NULL DEFAULT nextval('workflow_version_seq'),
  "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE ("name", "version")
);

CREATE INDEX idx_workflows_name ON workflows ("name", "version");

CREATE INDEX idx_workflows_id ON workflows ("id");

-- packages
CREATE TABLE packages (
  "id" UUID PRIMARY KEY,
  "name" VARCHAR(255) NOT NULL,
  "description" VARCHAR(255) NOT NULL,
  "code" TEXT NOT NULL,
  "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- process
CREATE TABLE processes (
  "id" UUID PRIMARY KEY,
  "workflow_id" UUID,
  "blueprint_spec" JSONB NOT NULL,
  "current_status" VARCHAR(255) NOT NULL,
  "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_workflow FOREIGN KEY(workflow_id) REFERENCES workflows(id)
);

-- process state
CREATE TABLE process_states (
  "id" UUID PRIMARY KEY,
  "process_id" UUID,
  "engine_id" UUID NOT NULL,
  "step_number" INT NOT NULL,
  "node_id" VARCHAR(255) NOT NULL,
  "next_node_id" VARCHAR(255),
  "bag" JSONB NOT NULL,
  "actor_data" JSONB NOT NULL,
  "external_input" JSONB,
  "result" JSONB,
  "error" TEXT,
  "status" VARCHAR(255) NOT NULL,
  "time_elapsed" BIGINT NOT NULL,
  "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_process FOREIGN KEY(process_id) REFERENCES processes(id)
);

ALTER TABLE
  processes
ADD
  COLUMN "current_state_id" UUID;

ALTER TABLE
  processes
ADD
  CONSTRAINT fk_process_state FOREIGN KEY(current_state_id) REFERENCES process_states(id);

CREATE INDEX idx_process_state_process_id ON process_states ("process_id");

-- activity manager
CREATE TABLE activity_managers (
  "id" UUID PRIMARY KEY,
  "process_state_id" UUID,
  "type" VARCHAR(255) NOT NULL,
  "props" JSONB NOT NULL,
  "parameters" JSONB NOT NULL,
  "status" VARCHAR(255) NOT NULL,
  "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_process_state FOREIGN KEY(process_state_id) REFERENCES process_states(id)
);

CREATE INDEX idx_am_started ON activity_managers (created_at ASC)
WHERE
  status = 'started';

CREATE INDEX idx_am_action ON activity_managers ((props ->> 'action'), created_at);

-- activity manager
CREATE TABLE activities (
  "id" UUID PRIMARY KEY,
  "activity_manager_id" UUID,
  "actor_data" JSONB NOT NULL,
  "data" JSONB NOT NULL,
  "status" VARCHAR(255) NOT NULL,
  "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_activity_manager FOREIGN KEY(activity_manager_id) REFERENCES activity_managers(id)
);

CREATE TABLE timers (
  "id" UUID PRIMARY KEY,
  "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "expires_at" TIMESTAMP NOT NULL,
  "active" BOOLEAN NOT NULL DEFAULT false,
  "resourse_type" VARCHAR(255) NOT NULL,
  "resource_id" UUID NOT NULL,
  "params" JSONB NOT NULL,
  "fired_at" TIMESTAMP
);

CREATE INDEX idx_timer_expiration ON timers (expires_at DESC)
WHERE
  active = true;