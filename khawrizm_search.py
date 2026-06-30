import sqlite3
import os

md_file = "Master_Ring0_Merged.md"
db_file = "Khawrizm_Omni.db"

def build_engine():
    print("[*] جاري أرشفة مليون سطر وبناء محرك البحث السيادي...")
    conn = sqlite3.connect(db_file)
    c = conn.cursor()
    c.execute("DROP TABLE IF EXISTS omni_docs")
    c.execute("CREATE VIRTUAL TABLE omni_docs USING fts5(title, content)")
    
    with open(md_file, 'r', encoding='utf-8', errors='ignore') as f:
        content = f.read()
    
    # تقسيم الملف الضخم بناءً على العناوين الرئيسية (المصادر)
    chunks = content.split('\n# ')
    
    for chunk in chunks:
        if not chunk.strip(): continue
        lines = chunk.split('\n', 1)
        title = lines[0][:200] # أخذ العنوان
        body = lines[1] if len(lines) > 1 else ""
        c.execute("INSERT INTO omni_docs (title, content) VALUES (?, ?)", (title, body))
        
    conn.commit()
    print(f"[+] تم فهرسة {len(chunks)} وثيقة ضخمة بنجاح.")
    return conn

def search_engine(conn, query):
    c = conn.cursor()
    c.execute("SELECT title, snippet(omni_docs, 1, '>>', '<<', '...', 64) FROM omni_docs WHERE omni_docs MATCH ? ORDER BY rank LIMIT 5", (query,))
    results = c.fetchall()
    print(f"\n[?] نتائج البحث عن: {query}\n" + "="*50)
    for res in results:
        print(f"📄 المصدر: {res[0]}\n🔍 المقتطف: {res[1]}\n{'-'*50}")

if __name__ == "__main__":
    if not os.path.exists(db_file):
        conn = build_engine()
    else:
        conn = sqlite3.connect(db_file)
    
    while True:
        q = input("\n[Khawrizm-Search] أدخل كلمة البحث (أو exit للخروج): ")
        if q.lower() == 'exit': break
        try: search_engine(conn, q)
        except Exception as e: print(f"[!] خطأ في البحث: {e}")
