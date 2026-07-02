import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

export default function App() {
  const [lang, setLang] = useState<'ar' | 'en'>('ar');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');

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
        title: lang === 'ar' ? 'مصنف عمل جديد - الخوارزمي' : 'New Workbook - Khawrizm',
        showinfobar: false, 
        showsheetbar: true,
        showstatisticBar: true,
        enableAddRow: true,
        enableAddBackTop: true,
        forceCalculation: true,
        plugins: ['chart'], 
        showtoolbarConfig: {
          undoRedo: true, paintFormat: true, currencyFormat: true, percentageFormat: true, numberDecrease: true, numberIncrease: true,
          moreFormats: true, font: true, fontSize: true, bold: true, italic: true, strikethrough: true, underline: true, textColor: true,
          fillColor: true, border: true, mergeCell: true, horizontalAlignMode: true, verticalAlignMode: true, textWrapMode: true,
          textDirectionMode: true, image: true, link: true, chart: true, postil: true, pivotTable: true, function: true, frozenMode: true,
          sortAndFilter: true, conditionalFormat: true, dataVerification: true, splitColumn: true, screenshot: true,
          findAndReplace: true, protection: true, print: true
        },
        hook: {
          // Fixed TS6133 by prefixing unused parameters with underscores
          cellUpdated: async (_r: number, _c: number, _oldValue: any, newValue: any) => {
            if(newValue && newValue.v && String(newValue.v).startsWith('=')) {
                try { await invoke('evaluate_xcv', { formula: String(newValue.v) }); } catch (e) {}
            }
          }
        }
      });
    }
  }, [lang, theme]); 

  const insertMathSymbol = (symbol: string) => {
    // @ts-ignore
    if (window.luckysheet) window.luckysheet.setCellValue(0, 0, symbol); 
  };

  return (
    <div className={`enterprise-app ${theme}`} dir={lang === 'ar' ? 'rtl' : 'ltr'}>
      <div className="excel-topbar">
        <div className="topbar-left">
          <span className="brand-icon">📊</span>
          <span className="brand-title">Khawrizm Analytics</span>
        </div>
        
        <div className="topbar-center">
          <input 
            type="text" 
            className="search-bar" 
            placeholder={lang === 'ar' ? 'بحث في الدوال والمعادلات (Alt+Q)' : 'Search Functions (Alt+Q)'} 
          />
        </div>
        
        <div className="topbar-right">
          <div className="math-toolbar">
            <button onClick={() => insertMathSymbol('∑')}>∑</button>
            <button onClick={() => insertMathSymbol('√')}>√</button>
            <button onClick={() => insertMathSymbol('π')}>π</button>
            <button onClick={() => insertMathSymbol('∞')}>∞</button>
            <button onClick={() => insertMathSymbol('∫')}>∫</button>
            <button onClick={() => insertMathSymbol('≈')}>≈</button>
          </div>

          <button className="control-btn" onClick={() => setTheme(theme === 'light' ? 'dark' : 'light')}>
            {theme === 'light' ? '🌙' : '☀️'}
          </button>
          <button className="control-btn" onClick={() => setLang(lang === 'ar' ? 'en' : 'ar')}>
            {lang === 'ar' ? 'EN' : 'عربي'}
          </button>
        </div>
      </div>
      
      <div id="khawrizm-grid" className="grid-container"></div>
    </div>
  );
}
