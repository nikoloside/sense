/* UI language (0037). The code keeps its Chinese strings (several double as protocol); text is swapped on display.
   Never touched: anything under [translate="no"], text inside 「」『』《》, and sentences that stay mostly Chinese
   after known phrases are swapped (that is content the engine wrote, not interface). */
const I18N=/*__I18N__*/{};
let UI_LANG="zh",I18N_T=null,I18N_RE=null,I18N_N=null;
const i18nNorm=s=>s.replace(/^[\s」』》·・:：、，,]+|[\s「『《·・:：、，,]+$/g,"");
const I18N_CJK=/[぀-ヿ一-鿿]/,I18N_HAN=/[一-鿿]/g;
/* single characters that are words on their own (tiers, the cap counter), only when not inside a Chinese run */
const I18N_ONE={en:{石:"Stone",铜:"Bronze",银:"Silver",金:"Gold",留:"Kept",跟:"Following",卡:"card"},ja:{铜:"銅",银:"銀",留:"保持",跟:"フォロー",卡:"カード"}};
const I18N_NUM={en:[[/(\d+)\s*张卡/g,"$1 cards"],[/(\d+)\s*张/g,"$1 "],[/(\d+)\s*次/g,"$1×"],[/(\d+)\s*秒/g,"$1 s"],[/(\d+)\s*天/g,"$1 d"],[/(\d+)\s*[条个道]/g,"$1"]],
  ja:[[/(\d+)\s*张/g,"$1枚"],[/(\d+)\s*次/g,"$1回"],[/(\d+)\s*秒/g,"$1秒"],[/(\d+)\s*天/g,"$1日"],[/(\d+)\s*条/g,"$1件"],[/(\d+)\s*个/g,"$1個"],[/(\d+)\s*道/g,"$1問"]]};
function i18nSet(lang){lang=I18N[lang]?lang:"zh";if(lang===UI_LANG)return;UI_LANG=lang;document.documentElement.lang=lang==="zh"?"zh-CN":lang;
  I18N_T=null;I18N_RE=null;if(lang==="zh")return;
  I18N_T={};for(const[k,v]of Object.entries(I18N[lang]))if(typeof v==="string")I18N_T[k.trim()]=v.trim();
  const ks=Object.keys(I18N_T).filter(k=>k.length>=2&&I18N_CJK.test(k)).sort((a,b)=>b.length-a.length);
  I18N_RE=new RegExp(ks.map(k=>k.replace(/[.*+?^${}()|[\]\\]/g,"\\$&")).join("|"),"g");
  I18N_N={};for(const[k,v]of Object.entries(I18N_T)){const a=i18nNorm(k);if(a&&I18N_N[a]==null)I18N_N[a]=i18nNorm(v)||v;}
  if(document.body)i18nWalk(document.body);if(document.title)document.title=i18nStr(document.title);}
function i18nSeg(seg){if(!I18N_CJK.test(seg))return seg;const k=seg.trim();if(I18N_T[k]!=null)return seg.replace(k,I18N_T[k]);
  const n=i18nNorm(seg);if(n&&I18N_N[n]!=null)return seg.replace(n,I18N_N[n]);
  /* swap known pieces for placeholders first, so what is left over is only the untranslated Chinese (Japanese output has kanji too) */
  const out=[],ph=v=>{out.push(v);return"\u0001"+(out.length-1)+"\u0002";};
  const han=(seg.match(I18N_HAN)||[]).length;let t=seg.replace(I18N_RE,m=>ph(I18N_T[m]));
  (I18N_NUM[UI_LANG]||[]).forEach(([re,r])=>{t=t.replace(re,(...a)=>ph(r.replace("$1",a[1])));});
  const one=I18N_ONE[UI_LANG]||{};t=t.replace(/[一-鿿]+/g,run=>run.length===1&&one[run]!=null?ph(one[run]):run);
  const left=(t.match(I18N_HAN)||[]).length;if(left)return seg;
  t=t.replace(/\u0001(\d+)\u0002/g,(_,i)=>out[i]);
  if(UI_LANG==="en")t=t.replace(/([a-z0-9])(card|cards)\b/g,"$1 $2").replace(/(\d)([A-Z])/g,"$1 $2").replace(/ {2,}/g," ");
  return t;}
function i18nStr(s){if(!I18N_T||!s||!I18N_CJK.test(s))return s;const k=s.trim();if(I18N_T[k]!=null)return s.replace(k,I18N_T[k]);
  /* inside quotes stays as is, unless it is exactly a known interface word (a button name like 「融合…」) */
  return s.split(/(「[^」]*」|『[^』]*』|《[^》]*》)/).map((seg,i)=>{if(!(i%2))return i18nSeg(seg);const in_=seg.slice(1,-1);return in_.length<=10&&I18N_T[in_]!=null?seg[0]+I18N_T[in_]+seg.slice(-1):seg;}).join("");}
const I18N_ATTR=["aria-label","title","placeholder"];
function i18nSkip(el){return!el||!!(el.closest&&el.closest('[translate="no"],script,style'));}
function i18nWalk(n){if(!I18N_T||!n)return;
  if(n.nodeType===3){if(n.parentElement&&n.parentElement.tagName!=="TEXTAREA"&&!i18nSkip(n.parentElement)){const v=i18nStr(n.nodeValue);if(v!==n.nodeValue)n.nodeValue=v;}return;}
  if(n.nodeType!==1||i18nSkip(n))return;
  for(const a of I18N_ATTR)if(n.hasAttribute(a)){const o=n.getAttribute(a),v=i18nStr(o);if(v!==o)n.setAttribute(a,v);}
  for(const c of n.childNodes)i18nWalk(c);}
new MutationObserver(ms=>{if(!I18N_T)return;for(const m of ms){if(m.type==="childList")m.addedNodes.forEach(i18nWalk);else i18nWalk(m.target);}})
  .observe(document.documentElement,{subtree:true,childList:true,characterData:true,attributes:true,attributeFilter:I18N_ATTR});
