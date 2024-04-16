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
1. Jika BambangShop memiliki beberapa jenis subscriber, misalnya subscriber email, subscriber SMS atau subscriber notifikasi push, penggunaan _interface_ atau _trait_ akan bermanfaat. Ini memungkinkan _programmer_ memiliki implementasi subscriber konkret yang berbeda sambil memastikan bahwa semuanya mematuhi kontrak yang sama. Namun, jika BambangShop hanya memiliki satu jenis subscriber dan tidak mengantisipasi memiliki jenis lain di masa depan, penggunaan sebuah _struct_ tunggal sudah cukup. Dalam hal ini, _programmer_ akan mendefinisikan _field_ dan metode yang diperlukan langsung dalam _struct_ Model tanpa memerlukan _interface_ atau _trait_ seperti yang sudah dilakukan pada tutorial ini.

2. Menggunakan Vec (daftar) mungkin sudah cukup jika jumlah Subscriber tidak terlalu besar dan _programmer_ hanya perlu mencari berdasarkan _id_ atau _url_ yang unik. Namun, jika program diharapkan memiliki kinerja yang lebih baik dalam mencari, memperbarui, atau menghapus Subscriber berdasarkan _id_ atau _url_, penggunaan DashMap seperti yang sudah gunakan saat ini lebih diperlukan.

DashMap akan memberikan kinerja yang lebih baik dalam operasi-operasi tersebut karena ia memungkinkan akses paralel dan tidak memerlukan penguncian seluruh struktur data saat mengaksesnya. Ini penting terutama jika kita mengharapkan jumlah Subscriber yang besar atau aplikasi akan berjalan di lingkungan yang bersifat konkuren.

Jadi, jika kita membutuhkan kinerja yang lebih baik dalam mengelola dan mengakses data Subscriber, menggunakan DashMap bisa menjadi pilihan yang lebih baik.

3. Dalam pola desain _Singleton_, tujuan utamanya adalah untuk memastikan bahwa suatu kelas hanya memiliki satu _instance_ dan menyediakan cara global untuk mengakses _instance_ tersebut. Namun, penggunaan pola _Singleton_ saja tidak cukup untuk memastikan keamanan _thread_. Meskipun _Singleton_ dapat memastikan bahwa hanya ada satu _instance_ dari struktur data, hal itu tidak menjamin keamanan operasi pada struktur data tersebut ketika digunakan dalam lingkungan _threading_. Oleh karena itu, menggunakan DashMap atau struktur data yang aman untuk _threading_ tetap diperlukan.

#### Reflection Publisher-2
1. Dengan memisahkan tanggung jawab Model ke dalam "Service" untuk logika bisnis dan "Repository" untuk akses data, kita dapat mencapai desain yang lebih modular, terpisah, dan dapat dipelihara. Hal ini meningkatkan keterbacaan, skalabilitas, dan fleksibilitas aplikasi kita, serta memungkinkan pengembangan yang lebih mudah di masa mendatang.

2. Jika kita hanya menggunakan Model tanpa memisahkan tanggung jawab ke dalam "Service" dan "Repository", kita mungkin akan mengalami beberapa masalah, seperti kesulitan dalam pengujian dan Kesulitan dalam perubahan dan pengembangan. Ketika perubahan atau penambahan fitur diperlukan, tanpa pemisahan yang jelas, kita mungkin akan menghadapi kesulitan dalam memahami dampak perubahan tersebut terhadap kode yang ada. Selain itu, penambahan fitur baru juga dapat menjadi lebih sulit karena tidak ada pemisahan yang jelas antara bagian-bagian yang berbeda dari aplikasi. Berikut merupakan contoh dari setiap interaksi,

    -  **Program Model**: Model Program bertanggung jawab atas data terkait program, seperti nama program, tanggal, dan detail lainnya. Tanpa pemisahan yang jelas, Model Program akan menjadi tempat untuk tidak hanya menyimpan data tetapi juga menjalankan operasi logika bisnis terkait program, seperti pembaruan status program, penjadwalan notifikasi, dan sebagainya. Hal ini dapat meningkatkan kompleksitas kode dan mengaburkan tanggung jawab Model.

    - **Subscriber Model**: Model Subscriber, tanpa pemisahan yang jelas, juga akan menjadi tempat untuk menyimpan data pengguna langganan dan juga melaksanakan logika bisnis terkait, seperti mengirim notifikasi ke pelanggan tertentu, memperbarui detail langganan, dan sebagainya. Ini dapat meningkatkan kompleksitas kode dalam Model Subscriber.

    - **Notification Model**: Model Notification akan bertanggung jawab atas penyimpanan dan pengelolaan notifikasi yang akan dikirim ke pelanggan. Tanpa pemisahan yang jelas, Model Notification juga mungkin akan menjadi tempat untuk menjalankan logika bisnis terkait notifikasi, seperti penjadwalan pengiriman notifikasi, pelacakan status pengiriman, dan sebagainya. Ini juga dapat meningkatkan kompleksitas kode dalam Model Notification.

3. Berikut merupakan beberapa fitur dari Postman yang sekiranya akan cukup bermanfaat ketika sedang mengerjakan satu proyek,
    - **Pengujian API**: Postman memungkinkan pengguna untuk menguji _endpoint_ API. Saya dapat membuat permintaan HTTP GET, POST, PUT, DELETE, dan lainnya dengan mudah dan melihat respons yang diterima. Ini memungkinkan saya untuk memverifikasi bahwa API berfungsi seperti yang diharapkan dan merespons dengan benar terhadap permintaan yang dikirim.
    - **Pemantauan Kinerja**: Postman juga dapat digunakan untuk memantau kinerja API dengan fitur Monitor. Ini memungkinkan saya untuk menjalankan skrip pengujian secara terjadwal dan secara otomatis memantau kinerja API dari waktu ke waktu. Saya dapat menerima laporan dan pemberitahuan jika ada masalah dengan kinerja API.
    - **Kolaborasi Tim**: Fitur Tim di Postman memungkinkan anggota tim untuk bekerja sama dalam pengembangan dan pengujian API. Saya dapat berbagi koleksi, lingkungan, dan skenario pengujian dengan rekan tim, serta memberikan komentar dan umpan balik langsung dalam aplikasi.

#### Reflection Publisher-3
1. 
2. 
3. 
