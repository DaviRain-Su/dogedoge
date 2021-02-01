CREATE TABLE `registers` (
    `id` varchar(50) NOT NULL DEFAULT '' COMMENT '唯一活动码',
    `phone_number` varchar(255) NOT NULL,
    `password` varchar(255) NOT NULL,
    `web3_address` varchar(255) NOT NULL,
    PRIMARY KEY (`id`) USING BTREE
);

INSERT INTO `registers`  VALUES ('1', '17366503261', '123456', '12bzRJfh7arnnfPPUZHeJUaE62QLEwhK48QnH9LXeK2m1iZU'),
                                ('2', '1025185920', '123456', '12bzRJfh7arnnfPPUZHeJUaE62QLEwhK48QnH9LXeK2m1iZU'),
                                ('3', '17366503261', '123456', '12bzRJfh7arnnfPPUZHeJUaE62QLEwhK48QnH9LXeK2m1iZU');