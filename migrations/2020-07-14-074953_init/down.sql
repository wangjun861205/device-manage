-- This file should undo anything in `up.sql`
SET FOREIGN_KEY_CHECKS = 0;

DROP TABLE IF EXISTS `device_info`;

DROP TABLE IF EXISTS `device`;

DROP TABLE IF EXISTS `subsystem_info`;

DROP TABLE IF EXISTS `subsystem`;

DROP TABLE IF EXISTS `component_info`;

DROP TABLE IF EXISTS `component`;

DROP TABLE IF EXISTS `device_info__subsystem_info`;

DROP TABLE IF EXISTS `subsystem_info__component_info`;

SET FOREIGN_KEY_CHECKS = 1;