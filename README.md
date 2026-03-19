# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1

Pertanyaan 1

Do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model struct is enough?


Dalam kasus BambangShop ini, satu struct Subscriber sudah cukup tanpa perlu trait. Pada Observer pattern di textbook, Subscriber biasanya berupa interface agar publisher bisa memanggil update() tanpa peduli implementasinya, bisa saja ada subscriber tipe email, SMS, webhook, dll. Namun di BambangShop, semua subscriber bersifat homogen yang artinya semuanya menerima notifikasi lewat HTTP POST ke URL masing-masing. Karena hanya ada satu jenis subscriber dengan satu perilaku yang konsisten, menambahkan trait hanya akan menambah abstraksi yang tidak diperlukan.


Pertanyaan 2

Is using Vec sufficient or using DashMap is necessary?


Vec tidak cukup untuk kasus ini. Karena id pada Product dan url pada Subscriber dimaksudkan sebagai identifier yang unik, kita butuh cara yang efisien untuk mengecek duplikasi dan melakukan pencarian. Dengan Vec, untuk mengecek apakah sebuah URL sudah ada atau menghapus subscriber tertentu, kita harus iterasi seluruh list dengan kompleksitas O(n). DashMap bekerja seperti HashMap dengan key-value pair, sehingga lookup dan delete by URL bisa dilakukan dalam O(1). Ini membuat DashMap menjadi pilihan yang lebih tepat dan memang diperlukan di sini.


Pertanyaan 3

Do we still need DashMap or can we implement Singleton pattern instead?


Kita tetap butuh DashMap meskipun sudah menggunakan Singleton pattern. Singleton hanya menjamin bahwa hanya ada satu instance dari SUBSCRIBERS di seluruh program, yang sebetulnya sudah terpenuhi lewat lazy_static. Tapi Singleton tidak menyelesaikan masalah thread-safety. Di Rust, jika banyak thread mencoba membaca dan menulis ke HashMap biasa secara bersamaan, bisa terjadi data race. DashMap dirancang khusus untuk concurrent access dengan internal locking per-shard, sehingga aman dipakai dari banyak thread sekaligus. Jadi DashMap bukan alternatif dari Singleton, melainkan pelengkap yang menangani thread-safety yang tidak bisa ditangani Singleton sendirian.

#### Reflection Publisher-2
Pertanyaan 1

Why do we need to separate "Service" and "Repository" from a Model?


Pemisahan "Service" dan "Repository" dari Model diperlukan karena prinsip Single Responsibility. Dalam MVC murni, Model menanggung terlalu banyak tanggung jawab sekaligus, yaitu menyimpan data, mengakses database, sekaligus menangani business logic. Ini melanggar prinsip SRP (Single Responsibility Principle). Dengan memisahkan Repository, kita punya satu lapisan khusus yang hanya bertanggung jawab mengakses dan memanipulasi data (CRUD). Dengan memisahkan Service, kita punya lapisan khusus yang hanya menangani business logic. Hasilnya, setiap lapisan lebih mudah ditest secara independen, lebih mudah diganti implementasinya tanpa merusak lapisan lain, dan kode lebih mudah dipahami karena setiap file punya tanggung jawab yang jelas.


Pertanyaan 2

What happens if we only use the Model? How does it affect code complexity?


Kalau kita hanya menggunakan Model tanpa Service dan Repository, setiap model akan sangat bergantung satu sama lain. Misalnya, Notification perlu tahu cara mengakses data Subscriber, dan Subscriber perlu tahu cara membuat Notification. Ini menciptakan tight coupling antar model. Bayangkan Program harus memanggil langsung method milik Subscriber untuk mengirim notifikasi, sementara Subscriber juga harus tahu struktur Notification untuk membentuk payload-nya. Kompleksitas tiap model akan meledak karena satu model harus mengurus banyak hal sekaligus. Perubahan kecil di satu model bisa berdampak ke model lain yang bergantung padanya, membuat kode sulit di-maintain dan sulit ditest.


Pertanyaan 3

How does Postman help you test your work?


Postman sangat membantu untuk menguji endpoint HTTP tanpa harus membuat frontend terlebih dahulu. Dengan Postman, saya bisa langsung mengirim request POST ke /notification/subscribe dengan body JSON berisi data subscriber, dan melihat response-nya secara real-time. Fitur Collection yang sudah disediakan di tutorial ini juga sangat berguna karena semua endpoint sudah tersusun rapi dan bisa langsung dipakai ulang. Untuk Group Project ke depannya, fitur Environment Variables di Postman juga menarik karena bisa menyimpan base URL dan token agar tidak perlu ditulis ulang di setiap request.

#### Reflection Publisher-3
Pertanyaan 1

Which variation of Observer Pattern do we use in this tutorial?

Dalam tutorial ini, saya menggunakan variasi Push model. Pada Push model, publisher (Main App) yang secara aktif mengirimkan data notifikasi langsung ke setiap subscriber saat event terjadi. Ketika ada product yang dibuat, dihapus, atau dipublish, Main App langsung memanggil notify() yang kemudian mengirim HTTP POST berisi payload notifikasi lengkap ke URL masing-masing subscriber. Subscriber tidak perlu meminta data apapun karena data sudah dikirimkan langsung oleh publisher begitu event terjadi.

Pertanyaan 2

What are the advantages and disadvantages if we used Pull model instead?

Kalau saya menggunakan Pull model sebagai gantinya, subscriber yang harus aktif meminta data ke publisher secara berkala (polling). Salah satu keuntungannya adalah subscriber bisa mengambil data sesuai kebutuhannya sendiri dan tidak akan dibanjiri notifikasi yang tidak relevan karena subscriber punya kontrol penuh atas kapan dan data apa yang diambil. Selain itu, publisher jadi lebih sederhana karena tidak perlu tahu format data apa yang dibutuhkan tiap subscriber, cukup menyediakan endpoint yang bisa di-query.
Namun kekurangannya cukup signifikan untuk kasus BambangShop ini. Subscriber harus terus-menerus melakukan polling ke publisher untuk mengecek apakah ada event baru, yang berarti banyak request sia-sia kalau memang tidak ada event sama sekali. Ini jelas boros resource baik di sisi publisher maupun subscriber. Selain itu notifikasinya juga tidak real-time karena ada jeda waktu antara event terjadi dan subscriber baru mengetahuinya, tergantung seberapa sering subscriber melakukan polling. Untuk kasus notification system seperti BambangShop, Push model jelas lebih cocok karena kita ingin subscriber langsung tahu begitu ada perubahan produk.

Pertanyaan 3

What will happen if we don't use multi-threading in the notification process?

Kalau saya tidak menggunakan multi-threading, proses notifikasi akan berjalan secara sequential satu per satu. Main App harus menunggu HTTP POST ke subscriber pertama benar-benar selesai sebelum bisa mulai mengirim ke subscriber berikutnya. Ini sangat bermasalah karena HTTP request itu inherently lambat dan sangat bergantung kondisi jaringan.
Bayangkan ada 100 subscriber yang terdaftar untuk satu product type. Main App harus menunggu 100 HTTP request selesai secara berurutan sebelum bisa memproses request lain dari user. Akibatnya response time Main App akan sangat lambat dan user yang mencoba membuat atau menghapus produk harus menunggu sangat lama hanya karena sistem sedang sibuk mengirim notifikasi. Bahkan kalau salah satu subscriber tidak merespons (timeout), seluruh proses notifikasi berikutnya ikut tertahan.
Dengan multi-threading seperti yang sudah saya implementasikan menggunakan thread::spawn, setiap notifikasi dikirim di thread terpisah secara paralel. Main App tidak perlu menunggu satu pun notifikasi selesai dan tetap responsif untuk melayani request user berikutnya.