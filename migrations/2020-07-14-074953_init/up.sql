-- Your SQL goes here

CREATE TABLE IF NOT EXISTS `device_info` (
    id int not null auto_increment comment 'id',
    name varchar(64) not null comment '名称',
    model varchar(255) not null comment '型号',
    maintain_interval int not null default 0 comment '维护间隔',
    create_at timestamp not null default current_timestamp comment '创建时间',
    update_at timestamp not null default current_timestamp on update current_timestamp comment '更新时间',
    primary key (id),
    unique key `uni_name_model` (name, model)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT '设备信息';

CREATE TABLE IF NOT EXISTS `device` (
    id int not null auto_increment comment 'id',
    name varchar(64) not null comment '名称',
    model varchar(255) not null comment '型号',
    maintain_interval int not null default 0 comment '维护间隔(小时)',
    unicode varchar(255) not null comment '唯一识别码',
    last_start_at datetime comment '最后的开车时间',
    last_stop_at datetime comment '最后停车时间',
    total_duration int not null default 0 comment '积累运行时间(小时)',
    status varchar(32) not null default 'Stopped' comment '状态: Running-运转; Stopped-停车; Breakdown-故障',
    create_at timestamp not null default current_timestamp comment '创建时间',
    update_at timestamp not null default current_timestamp on update current_timestamp comment '更新时间',
    primary key (id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT '设备';


CREATE TABLE IF NOT EXISTS `subsystem_info` (
    id int not null auto_increment comment 'id',
    name varchar(64) not null comment '名称',
    maintain_interval int not null default 0 comment '维护间隔(小时)',
    create_at timestamp not null default current_timestamp comment '创建时间',
    update_at timestamp not null default current_timestamp on update current_timestamp comment '更新时间',
    primary key (id),
    unique key `uni_name` (name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT '子系统信息';

CREATE TABLE IF NOT EXISTS `subsystem` (
    id int not null auto_increment comment 'id',
    device_id int not null comment '所属设备id',
    name varchar(64) not null comment '名称',
    maintain_interval int not null default 0 comment '维护间隔(小时)',
    create_at timestamp not null default current_timestamp comment '创建时间',
    update_at timestamp not null default current_timestamp on update current_timestamp comment '更新时间',
    primary key(id),
    unique key `uni_deviceid_name` (device_id, name),
    foreign key `fk_device` (device_id) references `device` (id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT '子系统';


CREATE TABLE IF NOT EXISTS `component_info` (
    id int not null auto_increment comment 'id',
    name varchar(64) not null comment '名称',
    model varchar(255) not null comment '型号',
    maintain_interval int not null default 0 comment '维护时间间隔(小时)',
    create_at timestamp not null default current_timestamp comment '创建时间',
    update_at timestamp not null default current_timestamp on update current_timestamp comment '更新时间',
    primary key (id),
    unique key `uni_name_model` (name, model)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT '零件信息';

CREATE TABLE IF NOT EXISTS `component` (
    id int not null auto_increment comment 'id',
    subsystem_id int not null comment '子系统id',
    name varchar(64) not null comment '名称',
    model varchar(255) not null comment '型号',
    maintain_interval int not null default 0 comment '维护间隔(小时)',
    create_at timestamp not null default current_timestamp comment '创建时间',
    update_at timestamp not null default current_timestamp on update current_timestamp comment '更新时间',
    primary key (id),
    foreign key `fk_subsystemid` (subsystem_id) references `subsystem` (id) on delete cascade
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT '零件';


CREATE TABLE IF NOT EXISTS `deviceinfo_subsysteminfo` (
    id int not null auto_increment comment 'id',
    device_info_id int not null comment '设备信息id',
    subsystem_info_id int not null comment '子系统信息id',
    primary key (id),
    foreign key `fk_device_info` (device_info_id) references `device_info` (id) on delete cascade,
    foreign key `fk_subsystem_info` (subsystem_info_id) references `subsystem_info` (id) on delete cascade,
    unique key `uni_deviceinfo_subsysteminfo` (device_info_id, subsystem_info_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT '设备信息与子系统信息多对多关系';

CREATE TABLE IF NOT EXISTS `subsysteminfo_componentinfo` (
    id int not null auto_increment comment 'id',
    device_info_id int not null comment '设备信息id',
    subsystem_info_id int not null comment '子系统信息id',
    component_info_id int not null comment '零部件信息id',
    quantity int not null default 1 comment '数量',
    primary key (id),
    unique key `uni_devinfo_subinfo_cominf` (device_info_id, subsystem_info_id, component_info_id),
    foreign key `fk_deviceinfo2` (device_info_id) references `device_info` (id) on delete cascade,
    foreign key `fk_subsysteminfo2` (subsystem_info_id) references `subsystem_info` (id) on delete cascade,
    foreign key `fk_componentinfo2` (component_info_id) references `component_info` (id) on delete cascade
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT '子系统信息与零部件信息多对多关系';