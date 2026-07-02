import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

export default function App() {
  const [lang, setLang] = useState<'ar' | 'en'>('ar');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [activeTab, setActiveTab] = useState<'home' | 'insert' | 'formulas' | 'data'>('home');

  useEffect(() => {
    document.documentElement.setAttribute('data-theme', theme);

    // @ts-ignore
    if (window.luckysheet) {
      // @ts-ignore
      window.luckysheet.destroy();
      
      // @ts-ignore
      window.luckysheet.create({
        container: 'khawrizm-grid',
        lang: lang,
        title: lang === 'ar' ? 'مصنف 1 - الخوارزمي إكسل' : 'Book 1 - Khawrizm Excel',
        showinfobar: false, // إخفاء الشريط الافتراضي لاستخدام الشريط المؤسسي المخصص
        showsheetbar: true,
        showstatisticBar: true,
        enableAddRow: true,
        enableAddBackTop: true,
        forceCalculation: true,
        plugins: ['chart'], // تفعيل محرك الرسوم البيانية المتقدم
        showtoolbarConfig: {
          undoRedo: true, paintFormat: true, currencyFormat: true, percentageFormat: true, numberDecrease: true, numberIncrease: true,
          moreFormats: true, font: true, fontSize: true, bold: true, italic: true, strikethrough: true, underline: true, textColor: true,
          fillColor: true, border: true, mergeCell: true, horizontalAlignMode: true, verticalAlignMode: true, textWrapMode: true,
          textDirectionMode: true, image: true, link: true, chart: true, postil: true, pivotTable: true, function: true, frozenMode: true,
          sortAndFilter: true, conditionalFormat: true, dataVerification: true, splitColumn: true, screenshot: true,
          findAndReplace: true, protection: true, print: true
        },
        hook: {
          cellUpdated: async (_r: number, _c: number, _oldValue: any, newValue: any) => {
            if(newValue && newValue.v && String(newValue.v).startsWith('=')) {
                try { await invoke('evaluate_xcv', { formula: String(newValue.v) }); } catch (e) {}
            }
          }
        }
      });
    }
  }, [lang, theme]);

  const insertFormula = (symbol: string) => {
    // @ts-ignore
    if (window.luckysheet) {
        // @ts-ignore
        window.luckysheet.setCellValue(0, 0, symbol);
    }
  };

  const t = {
    ar: { file: 'ملف', home: 'الصفحة الرئيسية', insert: 'إدراج', formulas: 'صيغ', data: 'بيانات', search: 'بحث عن الأدوات والميزات...', user: 'مستخدم' },
    en: { file: 'File', home: 'Home', insert: 'Insert', formulas: 'Formulas', data: 'Data', search: 'Search tools and features...', user: 'User' }
  };

  return (
    <div className={`enterprise-app ${theme}`} dir={lang === 'ar' ? 'rtl' : 'ltr'}>
      {/* شريط العنوان الأخضر الرسمي (Microsoft Standard) */}
      <div className="excel-topbar">
        <div className="topbar-left">
          <span className="brand-icon">📊</span>
          <span className="brand-title">Khawrizm Excel</span>
        </div>
        <div className="topbar-center">
          <input type="text" className="search-bar" placeholder={t[lang].search} />
        </div>
        <div className="topbar-right">
          <div className="account-circle"><span>{t[lang].user}</span></div>
          <div className="window-controls"><span>─</span><span>☐</span><span>✕</span></div>
        </div>
      </div>

      {/* شريط التبويبات (Ribbon Tabs) */}
      <div className="excel-ribbon-tabs">
        <div className="tab-item file-tab">{t[lang].file}</div>
        <div className={`tab-item ${activeTab === 'home' ? 'active' : ''}`} onClick={() => setActiveTab('home')}>{t[lang].home}</div>
        <div className={`tab-item ${activeTab === 'insert' ? 'active' : ''}`} onClick={() => setActiveTab('insert')}>{t[lang].insert}</div>
        <div className={`tab-item ${activeTab === 'formulas' ? 'active' : ''}`} onClick={() => setActiveTab('formulas')}>{t[lang].formulas}</div>
        <div className={`tab-item ${activeTab === 'data' ? 'active' : ''}`} onClick={() => setActiveTab('data')}>{t[lang].data}</div>
        
        {/* أدوات التحكم باللغة والتيم مخفية بشكل أنيق في يمين الشريط */}
        <div className="ribbon-controls">
           <button onClick={() => setTheme(theme === 'light' ? 'dark' : 'light')} className="theme-toggle">
             {theme === 'light' ? '🌙 نمط داكن' : '☀️ نمط فاتح'}
           </button>
           <button onClick={() => setLang(lang === 'ar' ? 'en' : 'ar')} className="lang-toggle">
             {lang === 'ar' ? 'English' : 'العربية'}
           </button>
        </div>
      </div>

      {/* شريط الأدوات الإضافي المخصص للمعادلات والبرمجة */}
      {activeTab === 'formulas' && (
        <div className="excel-ribbon-toolbar">
          <div className="toolbar-group">
            <button className="tool-btn" onClick={() => insertFormula('=SUM(')}>∑ الجمع التلقائي</button>
            <button className="tool-btn" onClick={() => insertFormula('=AVERAGE(')}>المتوسط</button>
            <span className="group-label">مكتبة الدالات</span>
          </div>
          <div className="toolbar-group">
            <button className="tool-btn" onClick={() => insertFormula('√')}>√ جذر</button>
            <button className="tool-btn" onClick={() => insertFormula('π')}>π باي</button>
            <button className="tool-btn" onClick={() => insertFormula('∫')}>∫ تكامل</button>
            <button className="tool-btn" onClick={() => insertFormula('=MATRIX(')}>[] مصفوفة</button>
            <span className="group-label">الرياضيات والبرمجة</span>
          </div>
        </div>
      )}

      {/* منطقة الجداول الحقيقية */}
      <div id="khawrizm-grid" className="grid-container"></div>
    </div>
  );
}
