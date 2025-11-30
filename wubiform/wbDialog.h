#ifndef WBDIALOG_H
#define WBDIALOG_H 1

#include <QDialog>

QT_BEGIN_NAMESPACE
namespace Ui { class wbForm; }
QT_END_NAMESPACE

class Q_DECL_HIDDEN wbDialog : public QDialog
{
	Q_OBJECT

public:
	wbDialog(QWidget * parent = nullptr);
	int  update_cbfunc(int which, void * cbfunc);
	void update_result(const char * res, unsigned int len);
	~wbDialog();

public slots:
	void text_arrived();
	void checked_recall(int changed);
	void clicked_load();
	void clicked_store();
	void clicked_next();
	void clicked_prev();

private:
	Ui::wbForm * ui;
	void * cbquery;
	void * cbload;
	void * cbstore;
	void * cbrecall;
	void * cb_hprev;
	void * cb_hnext;
	bool recall;
};
#endif
