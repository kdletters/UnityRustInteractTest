using System;
using Kdletters.EventSystem;
using Unity.VisualScripting;
using UnityEngine;
using UnityEngine.EventSystems;
using UnityEngine.UI;

namespace Minesweeper
{
    public class Block : MonoBehaviour, IPointerClickHandler
    {
        [SerializeField] private Button _btn;
        [SerializeField] private Text _text;

        private int _x;
        private int _y;

        private void Awake()
        {
            _btn.onClick.AddListener(OnClick);
            KEventSystem.Subscribe<RefreshBlock>(Refresh);
        }

        private void OnDestroy()
        {
            KEventSystem.Unsubscribe<RefreshBlock>(Refresh);
        }

        private void OnClick()
        {
            KEventSystem.Dispatch(new ClickBlock(_x, _y));
        }

        public void Set(int x, int y)
        {
            _x = x;
            _y = y;
        }

        private unsafe void Refresh(in RefreshBlock arg)
        {
            if (arg.Block->x != _x || arg.Block->y != _y)
            {
                return;
            }

            var isOpened = arg.Block->is_opened;
            _btn.targetGraphic.gameObject.SetActive(!isOpened);

            if (isOpened)
            {
                if (arg.Block->is_mine)
                {
                    _text.text = Minesweeper.Config.Mine;
                }
                else
                {
                    _text.text = arg.Block->num.ToString();
                }
            }
            else if (arg.Block->is_flag)
            {
                _text.text = Minesweeper.Config.Flag;
            }
            else
            {
                _text.text = string.Empty;
            }
        }

        public void OnPointerClick(PointerEventData eventData)
        {
            if (eventData.button == PointerEventData.InputButton.Right)
            {
                KEventSystem.Dispatch(new FlagBlock(_x, _y));
            }
        }
    }
}