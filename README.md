# Definition

Program ini berisi semua aturan dan logika untuk mengelola proses jual beli NFT (Non-Fungible Token) tanpa perlu perantara seperti bannk atau perusahaan terpusat. Semua transaksi dicatat secara permanan di blockchain dan dijalankan persis seperti yang tertulis di dalam kode. Tujuannya untuk menciptakan sebuah platform pasar NFT yang tersedentralisasi.

# Flow

Marketplace ini melibatkan tiga peran utama yaitu:

- Admin: Seseorang yang memiliki wewenang untuk menginisialisasi atau membuat sebuah marketplace baru. Mereka yang menentukan nama dan biaya (fee) awal untuk setiap transaksi di pasar tersebut.
- Penjual (Maker): Siapa pun yang memiliki NFT dan ingin menjualnya. Mereka akan mendaftarkan (listing) NFT mereka di marketplace dengan harga yang mereka tentukan sendiri.
- Pembeli (Taker): Siapa pun yang tertarik dan memiliki dana (SOL) untuk membeli NFT yang sudah terdaftar di marketplace.

Jadi bisa disimpulkan kalau Admin membuat pasar -> Penjual mendaftarkan NFT -> Pembeli membeli NFT. Program inilah yang menjadi pihak ketiga yang tak terlihat yang memastikan setiap langkah dalam alu ini berjalan dengan adil dan aman.

# How to run

1. Copy Metplex Program
   solana program dump -u m metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s .anchor/metaplex.so

2. Run Localnet Validator
   solana-test-validator -r --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s .anchor/metaplex.so

3. Build Anchor
   anchor build

4. Run Test
   anchor test
