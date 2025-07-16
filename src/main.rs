use rust_udp_utility::UdpHandler;
use gamepad_utility::GamePadsDriver;

fn main()
{
    let mut udp_ = UdpHandler::new("UdpTester", true);
    let mut gamepad = GamePadsDriver::new();
    
    udp_.open_set_address("192.168.11.6:64201", 1000);
    udp_.set_destination("192.168.11.100:8080");
    udp_.set_send_period(1);

    let mut c1 = 0.0;
    
    loop {
        gamepad.update();
        
        if gamepad.is_connected()
        {
            let controller = gamepad.get(1);

            c1 = controller.left_stick.y;
            let mut c2 = controller.right_stick.y;

            // if c1 < 0.0
            // {
            //     c1 = 0.0;
            // }
            if c1 > 1.0
            {
                c1 = 1.0;
            }
            if c2.abs() < 0.09
            {
                c2 = 0.0;
            }
            let v1 = (c1 * 127.0) + 127.0;
            let v2 = (c2 * 127.0) + 127.0;

            println!("{},{}", v1, v2);

            let mut buf = [0_u8; 10];
            buf[0] = v1 as u8;
            buf[1] = v2 as u8;
            udp_.send(&buf);
        }

        let _recv = udp_.recv();
    }
}