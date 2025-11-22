#include "wubiform.h"
#include "wbDialog.h"
#include "ui_wubiform.h"

#include <QApplication>
#include <QFileDialog>
#include <QScreen>

static QApplication * pQApp = nullptr;

wbDialog::wbDialog(QWidget * parent)
	: QDialog(parent), ui(new Ui::wbForm)
{
	// 初始化私有变量：
	cbquery  = nullptr;
	cbload   = nullptr;
	cbstore  = nullptr;
	cbnext   = nullptr;
	recall   = false;

	// 初始化窗口：
	ui->setupUi(this);
	// 默认不在回忆模式：
	checked_recall(0);

	// 窗体大小不可调整：
	this->setFixedSize(this->size());
}

wbDialog::~wbDialog()
{
	delete ui;
	ui       = nullptr;
	cbquery  = nullptr;
	cbload   = nullptr;
	cbstore  = nullptr;
	cbnext   = nullptr;
	recall   = false;
}

void wbDialog::checked_recall(int cst)
{
	recall = (cst != Qt::Unchecked);
	if (cst != ui->recall_mode->checkState())
		ui->recall_mode->setCheckState(cst ? Qt::Checked : Qt::Unchecked);
}

void wbDialog::clicked_load()
{
	WformButtonCb cb;
	cb = (WformButtonCb) cbload;
	if (cb == nullptr)
		return;

	QString histfile = QFileDialog::getOpenFileName(this,
		QString::fromUtf8("打开五笔查询历史"));
	if (histfile.isEmpty())
		return;

	QByteArray byts = histfile.toUtf8();
	const char * bytp = byts.constData();
	unsigned int bytl = (unsigned int) byts.size();
	if (bytp != nullptr && bytl > 0)
		cb(this, bytp, bytl);
}

void wbDialog::clicked_store()
{
	WformButtonCb cb;
	cb = (WformButtonCb) cbstore;
	if (cb == nullptr)
		return;

	QString histfile = QFileDialog::getSaveFileName(this,
		QString::fromUtf8("保存五笔查询历史"));
	if (histfile.isEmpty())
		return;

	QByteArray byts = histfile.toUtf8();
	const char * bytp = byts.constData();
	unsigned int bytl = (unsigned int) byts.size();
	if (bytp != nullptr && bytl > 0)
		cb(this, bytp, bytl);
}

void wbDialog::update_result(const char * res, unsigned int len)
{
	if (res == nullptr || len == 0) {
		ui->text_result->clear();
	} else {
		int rlen = (int) len;
		ui->text_result->setText(QString::fromUtf8(res, rlen));
	}
}

int wbDialog::update_cbfunc(int which, void * cbfunc)
{
	int ret = 0;
	switch (which) {
	case WFB_QUERY:
		this->cbquery  = cbfunc;
		break;
	case WFB_LOAD:
		this->cbload   = cbfunc;
		break;
	case WFB_STORE:
		this->cbstore  = cbfunc;
		break;
	case WFB_NEXT:
		this->cbnext   = cbfunc;
		break;
	default:
		ret = -1;
		break;
	}
	return ret;
}

void wbDialog::text_arrived()
{
	WformButtonCb cb;
	if (recall)
		cb = (WformButtonCb) cbnext;
	else
		cb = (WformButtonCb) cbquery;
	if (cb == nullptr)
		return;

	QString txt = ui->text_input->text();
	if (txt.isEmpty())
		return;

	QString hanzi = txt.trimmed();
	if (hanzi.isEmpty()) {
		ui->text_input->clear();
		return;
	}

	QByteArray byts = hanzi.toUtf8();
	const char * bytp = byts.constData();
	unsigned int bytl = (unsigned int) byts.size();
	if (bytp != nullptr && bytl > 0) {
		int ret = cb(this, bytp, bytl);
		if (recall || ret == 0)
			ui->text_input->clear();
	}
}

WubiForm wform_init_lib(void)
{
	int argc = 1;
	static char * argv[3];

	argv[0] = qstrdup("WubiDict");
	argv[1] = argv[2] = nullptr;
	pQApp = new QApplication(argc, argv);

	wbDialog * w = new wbDialog();
	return (WubiForm) w;
}

int wform_looping(WubiForm form)
{
	wbDialog * w = (wbDialog *) form;
	if (w == nullptr)
		return -1;

	w->show();
	if (pQApp != nullptr)
		return pQApp->exec();
	return -1;
}

int wform_push_result(WubiForm form,
	const char * utf8p, unsigned int utf8l)
{
	wbDialog * w;
	w = (wbDialog *) form;
	if (w != nullptr) {
		w->update_result(utf8p, utf8l);
		return 0;
	}
	return -1;
}

int wform_regiter_fun(WubiForm form, int which, WformButtonCb fcb)
{
	wbDialog * w;
	w = (wbDialog *) form;
	if (w != nullptr)
		return w->update_cbfunc(which, (void *) fcb);
	return -1;
}
