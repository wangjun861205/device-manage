-- This file should undo anything in `up.sql`
SET FOREIGN_KEY_CHECKS = 0;

DROP TABLE IF EXISTS `device_info`;

DROP TABLE IF EXISTS `device`;

DROP TABLE IF EXISTS `subsystem_info`;

DROP TABLE IF EXISTS `subsystem`;

DROP TABLE IF EXISTS `component_info`;

DROP TABLE IF EXISTS `component`;

DROP TABLE IF EXISTS `deviceinfo_subsysteminfo`;

DROP TABLE IF EXISTS `subsysteminfo_componentinfo`;

SET FOREIGN_KEY_CHECKS = 1;