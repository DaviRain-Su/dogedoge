CREATE TABLE `registers` (
    `id` varchar(50) NOT NULL DEFAULT '' COMMENT '唯一活动码',
    `uuid` varchar(100) NOT NULL,
    `phone_number` varchar(255) NOT NULL,
    `password` varchar(255) NOT NULL,
    `web3_address` varchar(255) NOT NULL,
    `sign_time` varchar(255) NOT NULL,
    `login_time` varchar(255) NOT NULL,
    PRIMARY KEY (`id`) USING BTREE
);

INSERT INTO `registers`  VALUES ('1', '239934993493', '17366503261', '123456', '12bzRJfh7arnnfPPUZHeJUaE62QLEwhK48QnH9LXeK2m1iZU', '2020-1-1', '2020-1-2'),
                                ('2', '239934993493', '1025185920', '123456', '12bzRJfh7arnnfPPUZHeJUaE62QLEwhK48QnH9LXeK2m1iZU', '2020-1-1', '2020-1-2'),
                                ('3', '239934993493', '17366503261', '123456', '12bzRJfh7arnnfPPUZHeJUaE62QLEwhK48QnH9LXeK2m1iZU','2020-1-1', '2020-1-2');