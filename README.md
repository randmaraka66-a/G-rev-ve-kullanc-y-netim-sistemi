# Görev Yönetim Sistemi (Task Manager)

## Proje Hakkında

Bu proje, Rust programlama dili kullanılarak geliştirilmiş bir Görev Yönetim Sistemi uygulamasıdır. Kullanıcıların görev eklemesine, listelemesine, silmesine ve tamamlamasına olanak sağlar.

Proje, Yazılım Mühendisliği dersi kapsamında sprint tabanlı geliştirme yaklaşımı kullanılarak hazırlanmıştır.

---

## Özellikler

* Görev ekleme
* Görev listeleme
* Görev silme
* Görev tamamlama
* Hatalı giriş kontrolü
* Dosyaya kayıt (kalıcı veri saklama)
* Program açılışında görevleri yükleme

---

## Kullanılan Teknolojiler

* Rust
* Cargo
* File I/O (Dosya İşlemleri)
* Git
* GitHub

---

## Proje Yapısı

```text
task_manager/
│
├── Cargo.toml
├── tasks.txt
└── src/
    └── main.rs
```

---

## Kurulum

Projeyi bilgisayarınıza klonlayın:

```bash
git clone <repo-link>
```

Proje klasörüne girin:

```bash
cd task_manager
```

Projeyi çalıştırın:

```bash
cargo run
```

---

## Kullanım

Program açıldığında aşağıdaki menü görüntülenir:

```text
1 - Görev Ekle
2 - Görevleri Listele
3 - Görev Sil
4 - Görev Tamamla
5 - Çıkış
```

Kullanıcı istediği işlemi seçerek görevlerini yönetebilir.

---

## Sprint Özeti

### Sprint 1

* Görev ekleme
* Görev listeleme

### Sprint 2

* Görev silme
* Giriş doğrulama

### Sprint 3

* Görev tamamlama
* Kodun fonksiyonlara ayrılması

### Sprint 4

* Dosya kaydetme sistemi
* Dosyadan veri yükleme
* Kalıcı veri saklama

---

## Öğrenilen Konular

* Struct kullanımı
* Vec veri yapısı
* Ownership ve Borrowing
* Dosya okuma ve yazma işlemleri
* Hata yönetimi
* Git ve GitHub kullanımı

---

## Gelecekteki Geliştirmeler

* Çok kullanıcılı kullanım desteği
* Veritabanı entegrasyonu
* Grafiksel kullanıcı arayüzü (GUI)
* Kullanıcı hesap sistemi
* Görev önceliklendirme

---

## Geliştirici

Rand Maraka

programlama Dilleri Projesi
Rust Görev Yönetim Sistemi
