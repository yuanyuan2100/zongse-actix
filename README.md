# A Blog
The project is running here: [zongse.de](https://zongse.de)

This project was my "entertainment" during the Covid-19 lockdown. Since the nature of my job doesn't allow me to WFH, I thought it was a good opportunity for me to learn something new. So I started learning Rust by building this blog website. 

I was inspired by many other Rust projects, such as [Kilerd/rubble](https://github.com/Kilerd/rubble) and [ramsayleung/blog](https://github.com/ramsayleung/blog). I asked some stupid questions and got a lot of warm help. But it was still too tough for me at the beginning. Before learning Rust, I had only known a little Python. Moreover, I had no idea about AWS, Linux, SQL, HTML, CSS, etc. There were so many times that I thought I can't build a real working programme. However, finally I'm here. Even though my project is still ugly and under construction, at least it is running now.

At first, I wrote a Rocket version, then I found it was not stable enough and will stop working after a couple of weeks running with no reasons. So I re-wrote the project by Actix. For this reason, the Diesel migration may not work properly since I am still using the database of the old Rocket project.

## Some "Unique Features"
* Google Login

   For ~~lazy~~ security reason, I didn't make user login functions. I am simply using Google login for guests. I don't save any guests' information in my system.

* Comment

   Almost all of those Rust blog projects I referred to do not have a comment function. Since this blog was designed as a playground for me and my friends, I've added the comment feature for them.

* Email Notification

   I will get an email when someone comments on my post so I don't need to check them manually every day. Especially when someone commented on a very old post. 
