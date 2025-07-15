-- Create database (if not exists)
CREATE DATABASE IF NOT EXISTS promptshelf DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- Select the database
USE promptshelf;

CREATE TABLE prompts (
    id             SERIAL PRIMARY KEY,         -- 自增主键
    latest_version VARCHAR(32),
    latest_commit  VARCHAR(64),       -- 最近一次提交的版本标识（可用 UUID 或 git hash）
    created_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    user_id        BIGINT,
    file_key       VARCHAR(100) NOT NULL,
    org_id         BIGINT
);

CREATE TABLE organizations (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    admin_id BIGINT NOT NULL,
    description TEXT,
    created_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE users (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(100) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    role VARCHAR(16) NOT NULL DEFAULT 'user',
    valid BOOLEAN NOT NULL DEFAULT TRUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE user_organizations (
    user_id BIGINT NOT NULL,
    org_id BIGINT NOT NULL,
    PRIMARY KEY (user_id, org_id),
    INDEX idx_org_id (org_id)
);
